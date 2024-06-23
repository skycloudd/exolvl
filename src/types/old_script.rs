use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Script {
    pub script_id: uuid::Uuid,
    pub name: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub actions: Vec<OldAction>,
}

impl Read for Script {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            script_id: Read::read(input)?,
            name: Read::read(input)?,
            creation_date: Read::read(input)?,
            actions: Read::read(input)?,
        })
    }
}

impl Write for Script {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.script_id.write(output)?;
        self.name.write(output)?;
        self.creation_date.write(output)?;
        self.actions.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OldAction {
    pub action_type: OldActionType,
    pub wait: bool,
    pub properties: Vec<OldActionProperty>,
}

impl Read for OldAction {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            action_type: Read::read(input)?,
            wait: Read::read(input)?,
            properties: Read::read(input)?,
        })
    }
}

impl Write for OldAction {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.action_type.write(output)?;
        self.wait.write(output)?;
        self.properties.write(output)
    }
}

macro_rules! define_old_action_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum OldActionType {
            $($name = $number),*
        }

        impl TryFrom<i32> for OldActionType {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(OldActionType::$name),)*
                    _ => Err(())
                }
            }
        }

        impl From<&OldActionType> for i32 {
            fn from(value: &OldActionType) -> Self {
                match value {
                    $(OldActionType::$name => $number,)*
                }
            }
        }
    };
}

define_old_action_type!(
    RunScript = 0,
    StopScripts = 1,
    Wait = 2,
    WaitFrames = 3,
    Move = 4,
    Jump = 5,
    Slam = 6,
    Charge = 7,
    Scale = 8,
    Rotate = 9,
    RotateAround = 10,
    SetDirection = 11,
    Activate = 12,
    Deactivate = 13,
    PlaySound = 14,
    PlayMusic = 15,
    SetCinematic = 16,
    SetInputEnabled = 17,
    PanCameraToObject = 18,
    CameraFollowPlayer = 19,
    ShowGameText = 20,
    SetVulnerable = 21,
    Color = 22,
    Damage = 23,
    Kill = 24,
    Finish = 25,
    SetGravity = 26,
    SetVelocity = 27
);

impl Read for OldActionType {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidDynamicType(value))
    }
}

impl Write for OldActionType {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::from(self).write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OldActionProperty {
    pub name: String,
    pub value: String,
}

impl Read for OldActionProperty {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            name: Read::read(input)?,
            value: Read::read(input)?,
        })
    }
}

impl Write for OldActionProperty {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.name.write(output)?;
        self.value.write(output)
    }
}
