use crate::{error::Error, Read, Write};

macro_rules! define_dynamic_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug,   Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum DynamicType {
            $($name = $number),*
        }

        impl TryFrom<i32> for DynamicType {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(DynamicType::$name),)*
                    _ => Err(())
                }
            }
        }

        impl From<&DynamicType> for i32 {
            fn from(value: &DynamicType) -> Self {
                match value {
                    $(DynamicType::$name => $number,)*
                }
            }
        }
    };
}

define_dynamic_type!(
    BoolConstant = 0,
    BoolVariable = 1,
    BoolNot = 2,
    BoolAnd = 3,
    BoolOr = 4,
    BoolEqualBool = 5,
    BoolEqualNumber = 6,
    BoolEqualString = 7,
    BoolEqualColor = 8,
    BoolEqualVector = 9,
    BoolEqualObject = 10,
    BoolNotEqualBool = 11,
    BoolNotEqualNumber = 12,
    BoolNotEqualString = 13,
    BoolNotEqualColor = 14,
    BoolNotEqualVector = 15,
    BoolNotEqualObject = 16,
    BoolLess = 17,
    BoolLessOrEqual = 18,
    BoolGreater = 19,
    BoolGreaterOrEqual = 20,
    BoolObjectDead = 21,
    BoolPlayerOnGround = 22,
    BoolPlayerOnWalljump = 23,
    BoolPlayerOnBooster = 24,
    BoolPlayerOnSwing = 25,
    BoolPlayerInFloatingZone = 26,
    BoolPlayerUsingGlider = 27,
    BoolObjectsColliding = 28,
    BoolInputPressed = 29,
    BoolInputPressedLeft = 30,
    BoolInputPressedRight = 31,
    BoolInputHeld = 32,
    BoolInputHeldLeft = 33,
    BoolInputHeldRight = 34,
    BoolInputReleased = 35,
    BoolInputReleasedLeft = 36,
    BoolInputReleasedRight = 37,
    IntConstant = 38,
    IntVariable = 39,
    IntAdd = 40,
    IntSubtract = 41,
    IntMultiply = 42,
    IntDivide = 43,
    IntModulo = 44,
    IntMin = 45,
    IntMax = 46,
    IntAbs = 47,
    IntSign = 48,
    IntRound = 49,
    IntCeil = 50,
    IntFloor = 51,
    IntRandom = 52,
    IntRepeatCount = 53,
    IntObjectDirection = 54,
    IntObjectSetCount = 55,
    FloatConstant = 56,
    FloatVariable = 57,
    FloatAdd = 58,
    FloatSubtract = 59,
    FloatMultiply = 60,
    FloatDivide = 61,
    FloatModulo = 62,
    FloatMin = 63,
    FloatMax = 64,
    FloatAbs = 65,
    FloatSign = 66,
    FloatRound = 67,
    FloatCeil = 68,
    FloatFloor = 69,
    FloatCos = 70,
    FloatSin = 71,
    FloatTan = 72,
    FloatAcos = 73,
    FloatAsin = 74,
    FloatAtan = 75,
    FloatSqrt = 76,
    FloatPow = 77,
    FloatRandom = 78,
    FloatTime = 79,
    FloatSemitones = 80,
    FloatVectorX = 81,
    FloatVectorY = 82,
    FloatVectorLength = 83,
    FloatVectorLengthSqr = 84,
    FloatVectorDistance = 85,
    FloatVectorDistanceSqr = 86,
    FloatVectorDot = 87,
    FloatVectorAngle = 88,
    FloatVectorAngleBetween = 89,
    FloatObjectRotation = 90,
    FloatObjectGlobalRotation = 91,
    FloatCameraViewportSize = 92,
    FloatDamageAmount = 93,
    StringConstant = 94,
    StringVariable = 95,
    StringFromInt = 96,
    StringFromFloat = 97,
    StringConcat = 98,
    ColorConstant = 99,
    ColorValues = 100,
    ColorVariable = 101,
    ColorObjectColor = 102,
    VectorConstant = 103,
    VectorValues = 104,
    VectorVariable = 105,
    VectorAdd = 106,
    VectorSubtract = 107,
    VectorMultiply = 108,
    VectorDivide = 109,
    VectorNormalize = 110,
    VectorPerpendicular = 111,
    VectorReflect = 112,
    VectorObjectPos = 113,
    VectorObjectGlobalPos = 114,
    VectorObjectScale = 115,
    VectorObjectGlobalScale = 116,
    VectorObjectVelocity = 117,
    VectorCameraPos = 118,
    SoundConstant = 119,
    SoundVariable = 120,
    MusicConstant = 121,
    MusicVariable = 122,
    ObjectConstant = 123,
    ObjectVariable = 124,
    ObjectAnyObject = 125,
    ObjectFirstFromSet = 126,
    ObjectRandomFromSet = 127,
    ObjectElementFromSet = 128,
    ObjectSourceObject = 129,
    ObjectCollidedObject = 130,
    ObjectTargetObject = 131,
    ObjectPlayer = 132,
    ObjectParent = 133,
    ObjectSetConstant = 134,
    ObjectSetVariable = 135,
    ObjectSetConcat = 136,
    ObjectSetPlayers = 137,
    ObjectSetObjectsWithTag = 138,
    TransitionConstant = 139,
    TransitionVariable = 140,
    EasingConstant = 141,
    EasingVariable = 142,
    ObjectSetChildren = 143,
    BoolObjectActivated = 144,
    FloatLevelTime = 145,
    BoolPlayerJumpLocked = 146,
    StringObjectTag = 147,
    SpriteConstant = 148,
    SpriteVariable = 149,
    ScriptConstant = 150,
    ScriptVariable = 151,
    BoolParameter = 152,
    IntParameter = 153,
    FloatParameter = 154,
    StringParameter = 155,
    ColorParameter = 156,
    VectorParameter = 157,
    SoundParameter = 158,
    MusicParameter = 159,
    ObjectParameter = 160,
    ObjectSetParameter = 161,
    TransitionParameter = 162,
    EasingParameter = 163,
    SpriteParameter = 164,
    ScriptParameter = 165,
    BoolObjectsCollidingWithPoint = 166,
    FloatRoundDecimals = 167,
    VectorPointerPositionDeprecated = 168,
    VectorPointerWorldPositionDeprecated = 169,
    VectorCollisionPoint = 170,
    VectorCollisionNormal = 171,
    ObjectRepeatObject = 172,
    VectorClosestFromPoint = 173,
    ObjectSetAllObjects = 174,
    ObjectSetObjectsInLayer = 175,
    ObjectSetObjectsInCircle = 176,
    LayerConstant = 177,
    LayerVariable = 178,
    LayerParameter = 179,
    VectorRotate = 180,
    IntLastSoundInstance = 181,
    ObjectSetUnion = 182,
    ObjectSetIntersection = 183,
    ObjectSetDifference = 184,
    ObjectSetRemoveAtIndex = 185,
    VectorPointerPosition = 186,
    VectorPointerWorldPosition = 187,
    BoolPointerDown = 188,
    BoolPointerHeld = 189,
    BoolPointerReleased = 190,
    FloatColorR = 191,
    FloatColorG = 192,
    FloatColorB = 193,
    FloatColorA = 194,
    StringSubstring = 195,
    IntStringLength = 196
);

impl Read for DynamicType {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidDynamicType(value))
    }
}

impl Write for DynamicType {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::from(self).write(output)
    }
}
