use deno_resolver::npm::{DenoInNpmPackageChecker, NpmResolver};
use deno_runtime::{
    deno_core::{FsModuleLoader, ModuleSpecifier, PollEventLoopOptions},
    deno_fs::RealFs,
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{MainWorker, WorkerOptions, WorkerServiceOptions},
};
use std::{future::Future, pin::Pin, rc::Rc, sync::Arc, task::Poll};
use tokio::{sync::mpsc, task::LocalSet};

#[derive(Debug)]
pub enum ScriptExecutorMessage {
    LoadModule { script: String },
}

/// Handle for accessing the script executor
#[derive(Clone)]
pub struct ScriptExecutorHandle {
    /// Channel for sending the execute message
    tx: mpsc::Sender<ScriptExecutorMessage>,
}

#[tokio::test]
async fn test() {
    let handle = create_script_executor();

    handle
        .tx
        .send(ScriptExecutorMessage::LoadModule {
            script: "console.log('loaded');".to_string(),
        })
        .await
        .unwrap();

    loop {
        spin_loop();
    }
}

/// Creates a dedicated thread for receiving script execution requests. The
/// thread will process the script execution requests providing the responses
///
/// The JS runtime is !Send and thus it cannot be shared across tokio async tasks
/// so here its provided a dedicated single threaded runtime and its own thread
pub fn create_script_executor() -> ScriptExecutorHandle {
    let (tx, rx) = mpsc::channel::<ScriptExecutorMessage>(5);

    std::thread::spawn(move || {
        // Create a new tokio runtime in the dedicated thread
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create script async runtime");

        let worker = MainWorker::bootstrap_from_options(
            &ModuleSpecifier::parse("file://").expect("invalid file"),
            WorkerServiceOptions::<
                DenoInNpmPackageChecker,
                NpmResolver<sys_traits::impls::RealSys>,
                sys_traits::impls::RealSys,
            > {
                module_loader: Rc::new(FsModuleLoader),
                permissions: PermissionsContainer::allow_all(Arc::new(
                    RuntimePermissionDescriptorParser::new(sys_traits::impls::RealSys),
                )),
                blob_store: Default::default(),
                broadcast_channel: Default::default(),
                feature_checker: Default::default(),
                node_services: Default::default(),
                npm_process_state_provider: Default::default(),
                root_cert_store_provider: Default::default(),
                fetch_dns_resolver: Default::default(),
                shared_array_buffer_store: Default::default(),
                compiled_wasm_module_store: Default::default(),
                v8_code_cache: Default::default(),
                fs: Arc::new(RealFs),
            },
            WorkerOptions {
                extensions: vec![],
                ..Default::default()
            },
        );

        runtime.block_on(ScriptExecutorFuture::new(worker, rx));
    });

    ScriptExecutorHandle { tx }
}

struct ScriptExecutorFuture {
    /// JS runtime task
    worker: MainWorker,

    /// Channel to receive execute messages from
    rx: mpsc::Receiver<ScriptExecutorMessage>,

    /// Local set for spawned promise tasks
    local_set: LocalSet,
}

impl ScriptExecutorFuture {
    pub fn new(runtime: MainWorker, rx: mpsc::Receiver<ScriptExecutorMessage>) -> Self {
        Self {
            worker: runtime,
            rx,
            local_set: LocalSet::new(),
        }
    }
}

impl Future for ScriptExecutorFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // Initial pass when not messages are available
        {
            // Poll the promises local set
            _ = Pin::new(&mut this.local_set).poll(cx);

            // Poll event loop for any promises
            let _ = this
                .worker
                .js_runtime
                .poll_event_loop(cx, PollEventLoopOptions::default());
        }

        // Poll incoming script execute messages
        while let Poll::Ready(msg) = this.rx.poll_recv(cx) {
            let msg = match msg {
                Some(msg) => msg,
                None => return Poll::Ready(()),
            };

            // PROCESS MESSAGE
            match msg {
                ScriptExecutorMessage::LoadModule { script } => {
                    this.worker
                        .js_runtime
                        .execute_script("[plugin]", script)
                        .unwrap();
                }
            }

            // Poll the promises local set
            _ = Pin::new(&mut this.local_set).poll(cx);

            // Poll the event loop
            let _ = this
                .worker
                .js_runtime
                .poll_event_loop(cx, PollEventLoopOptions::default());
        }

        Poll::Pending
    }
}
