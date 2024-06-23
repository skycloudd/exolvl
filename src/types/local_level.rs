use crate::{error::Error, Read, Write};
use uuid::Uuid;

/// The local level data for this level.
///
/// This data is only ever used in the level editor and is not uploaded to the server.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
pub struct LocalLevel {
    /// The version of the exolvl format that this level uses.
    ///
    /// The current latest serialization version is 18.
    pub serialization_version: i32,
    /// The UUID of the level.
    pub level_id: Uuid,
    /// The version of the level e.g. v1, v2, etc.
    pub level_version: i32,
    /// The name of the level.
    pub level_name: String,
    /// The base64-encoded data for the thumbnail of the level.
    pub thumbnail: String,
    /// When this level was created.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// When this level was last updated.
    pub update_date: chrono::DateTime<chrono::Utc>,
    /// The author medal time for this level in milliseconds.
    pub author_time: i64,
    /// The author medal lap times for this level in milliseconds.
    pub author_lap_times: Vec<i64>,
    /// The silver medal time for this level in milliseconds.
    pub silver_medal_time: i64,
    /// The gold medal time for this level in milliseconds.
    pub gold_medal_time: i64,
    /// The number of laps in this level.
    pub laps: i32,
    /// Whether this level is private or public.
    pub private: bool,

    /// If this is true, the level can be opened in the new level editor. Otherwise it's for the "legacy" editor.
    pub nova_level: bool,
}

impl Read for LocalLevel {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            serialization_version: Read::read(input)?,
            level_id: Read::read(input)?,
            level_version: Read::read(input)?,
            level_name: Read::read(input)?,
            thumbnail: Read::read(input)?,
            creation_date: Read::read(input)?,
            update_date: Read::read(input)?,
            author_time: Read::read(input)?,
            author_lap_times: Read::read(input)?,
            silver_medal_time: Read::read(input)?,
            gold_medal_time: Read::read(input)?,
            laps: Read::read(input)?,
            private: Read::read(input)?,
            nova_level: Read::read(input)?,
        })
    }
}

impl Write for LocalLevel {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.serialization_version.write(output)?;
        self.level_id.write(output)?;
        self.level_version.write(output)?;
        self.level_name.write(output)?;
        self.thumbnail.write(output)?;
        self.creation_date.write(output)?;
        self.update_date.write(output)?;
        self.author_time.write(output)?;
        self.author_lap_times.write(output)?;
        self.silver_medal_time.write(output)?;
        self.gold_medal_time.write(output)?;
        self.laps.write(output)?;
        self.private.write(output)?;
        self.nova_level.write(output)
    }
}
