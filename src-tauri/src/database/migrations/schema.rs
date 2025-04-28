use sea_query::{ColumnDef, IntoIden};

pub fn pk_uuid<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name)
        .uuid()
        .not_null()
        .primary_key()
        .to_owned()
}

pub fn pk_integer<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name)
        .integer()
        .not_null()
        .primary_key()
        .to_owned()
}

pub fn uuid<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).uuid().not_null().to_owned()
}

pub fn uuid_null<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).uuid().null().to_owned()
}

pub fn string<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).string().not_null().to_owned()
}

pub fn json<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).json_binary().not_null().to_owned()
}

pub fn integer<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).integer().not_null().to_owned()
}

pub fn date_time<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).date_time().not_null().to_owned()
}

pub fn boolean<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(name).boolean().not_null().to_owned()
}
