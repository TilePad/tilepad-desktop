// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function getErrorMessage(error: any) {
  if (error instanceof Error) {
    return error.message;
  }

  if (typeof error === "string") {
    return error;
  }

  if (error.message !== undefined && typeof error.message === "string") {
    return error.message;
  }

  return "Unknown error";
}

export function toastErrorMessage(title: string) {
  return (error: unknown) => {
    console.error(title, error);
    return `${title}: ${getErrorMessage(error)}`;
  };
}
