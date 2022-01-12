#[doc = "Register `CTRLR0` reader"]
pub struct R(crate::R<CTRLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLR0` writer"]
pub struct W(crate::W<CTRLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data Frame Size. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode, refer to Table 6-2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFS_A {
    #[doc = "3: 4 bit serial data transfer"]
    WORD_4BIT = 3,
    #[doc = "4: 5 bit serial data transfer"]
    WORD_5BIT = 4,
    #[doc = "5: 6 bit serial data transfer"]
    WORD_6BIT = 5,
    #[doc = "6: 7 bit serial data transfer"]
    WORD_7BIT = 6,
    #[doc = "7: 8 bit serial data transfer"]
    WORD_8BIT = 7,
    #[doc = "8: 9 bit serial data transfer"]
    WORD_9BIT = 8,
    #[doc = "9: 10 bit serial data transfer"]
    WORD_10BIT = 9,
    #[doc = "10: 11 bit serial data transfer"]
    WORD_11BIT = 10,
    #[doc = "11: 12 bit serial data transfer"]
    WORD_12BIT = 11,
    #[doc = "12: 13 bit serial data transfer"]
    WORD_13BIT = 12,
    #[doc = "13: 14 bit serial data transfer"]
    WORD_14BIT = 13,
    #[doc = "14: 15 bit serial data transfer"]
    WORD_15BIT = 14,
    #[doc = "15: 16 bit serial data transfer"]
    WORD_16BIT = 15,
}
impl From<DFS_A> for u8 {
    #[inline(always)]
    fn from(variant: DFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DFS` reader - Data Frame Size. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode, refer to Table 6-2."]
pub struct DFS_R(crate::FieldReader<u8, DFS_A>);
impl DFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFS_A> {
        match self.bits {
            3 => Some(DFS_A::WORD_4BIT),
            4 => Some(DFS_A::WORD_5BIT),
            5 => Some(DFS_A::WORD_6BIT),
            6 => Some(DFS_A::WORD_7BIT),
            7 => Some(DFS_A::WORD_8BIT),
            8 => Some(DFS_A::WORD_9BIT),
            9 => Some(DFS_A::WORD_10BIT),
            10 => Some(DFS_A::WORD_11BIT),
            11 => Some(DFS_A::WORD_12BIT),
            12 => Some(DFS_A::WORD_13BIT),
            13 => Some(DFS_A::WORD_14BIT),
            14 => Some(DFS_A::WORD_15BIT),
            15 => Some(DFS_A::WORD_16BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WORD_4BIT`"]
    #[inline(always)]
    pub fn is_word_4bit(&self) -> bool {
        **self == DFS_A::WORD_4BIT
    }
    #[doc = "Checks if the value of the field is `WORD_5BIT`"]
    #[inline(always)]
    pub fn is_word_5bit(&self) -> bool {
        **self == DFS_A::WORD_5BIT
    }
    #[doc = "Checks if the value of the field is `WORD_6BIT`"]
    #[inline(always)]
    pub fn is_word_6bit(&self) -> bool {
        **self == DFS_A::WORD_6BIT
    }
    #[doc = "Checks if the value of the field is `WORD_7BIT`"]
    #[inline(always)]
    pub fn is_word_7bit(&self) -> bool {
        **self == DFS_A::WORD_7BIT
    }
    #[doc = "Checks if the value of the field is `WORD_8BIT`"]
    #[inline(always)]
    pub fn is_word_8bit(&self) -> bool {
        **self == DFS_A::WORD_8BIT
    }
    #[doc = "Checks if the value of the field is `WORD_9BIT`"]
    #[inline(always)]
    pub fn is_word_9bit(&self) -> bool {
        **self == DFS_A::WORD_9BIT
    }
    #[doc = "Checks if the value of the field is `WORD_10BIT`"]
    #[inline(always)]
    pub fn is_word_10bit(&self) -> bool {
        **self == DFS_A::WORD_10BIT
    }
    #[doc = "Checks if the value of the field is `WORD_11BIT`"]
    #[inline(always)]
    pub fn is_word_11bit(&self) -> bool {
        **self == DFS_A::WORD_11BIT
    }
    #[doc = "Checks if the value of the field is `WORD_12BIT`"]
    #[inline(always)]
    pub fn is_word_12bit(&self) -> bool {
        **self == DFS_A::WORD_12BIT
    }
    #[doc = "Checks if the value of the field is `WORD_13BIT`"]
    #[inline(always)]
    pub fn is_word_13bit(&self) -> bool {
        **self == DFS_A::WORD_13BIT
    }
    #[doc = "Checks if the value of the field is `WORD_14BIT`"]
    #[inline(always)]
    pub fn is_word_14bit(&self) -> bool {
        **self == DFS_A::WORD_14BIT
    }
    #[doc = "Checks if the value of the field is `WORD_15BIT`"]
    #[inline(always)]
    pub fn is_word_15bit(&self) -> bool {
        **self == DFS_A::WORD_15BIT
    }
    #[doc = "Checks if the value of the field is `WORD_16BIT`"]
    #[inline(always)]
    pub fn is_word_16bit(&self) -> bool {
        **self == DFS_A::WORD_16BIT
    }
}
impl core::ops::Deref for DFS_R {
    type Target = crate::FieldReader<u8, DFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFS` writer - Data Frame Size. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode, refer to Table 6-2."]
pub struct DFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4 bit serial data transfer"]
    #[inline(always)]
    pub fn word_4bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_4BIT)
    }
    #[doc = "5 bit serial data transfer"]
    #[inline(always)]
    pub fn word_5bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_5BIT)
    }
    #[doc = "6 bit serial data transfer"]
    #[inline(always)]
    pub fn word_6bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_6BIT)
    }
    #[doc = "7 bit serial data transfer"]
    #[inline(always)]
    pub fn word_7bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_7BIT)
    }
    #[doc = "8 bit serial data transfer"]
    #[inline(always)]
    pub fn word_8bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_8BIT)
    }
    #[doc = "9 bit serial data transfer"]
    #[inline(always)]
    pub fn word_9bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_9BIT)
    }
    #[doc = "10 bit serial data transfer"]
    #[inline(always)]
    pub fn word_10bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_10BIT)
    }
    #[doc = "11 bit serial data transfer"]
    #[inline(always)]
    pub fn word_11bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_11BIT)
    }
    #[doc = "12 bit serial data transfer"]
    #[inline(always)]
    pub fn word_12bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_12BIT)
    }
    #[doc = "13 bit serial data transfer"]
    #[inline(always)]
    pub fn word_13bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_13BIT)
    }
    #[doc = "14 bit serial data transfer"]
    #[inline(always)]
    pub fn word_14bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_14BIT)
    }
    #[doc = "15 bit serial data transfer"]
    #[inline(always)]
    pub fn word_15bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_15BIT)
    }
    #[doc = "16 bit serial data transfer"]
    #[inline(always)]
    pub fn word_16bit(self) -> &'a mut W {
        self.variant(DFS_A::WORD_16BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `FRF` reader - 00 - Motorola SPI only"]
pub struct FRF_R(crate::FieldReader<u8, u8>);
impl FRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRF` writer - 00 - Motorola SPI only"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCPH_A {
    #[doc = "0: Serial clock toggles in middle of first data bit"]
    TOGGLE_MIDDLE = 0,
    #[doc = "1: Serial clock toggles at start of first data bit"]
    TOGGLE_START = 1,
}
impl From<SCPH_A> for bool {
    #[inline(always)]
    fn from(variant: SCPH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCPH` reader - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH."]
pub struct SCPH_R(crate::FieldReader<bool, SCPH_A>);
impl SCPH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCPH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCPH_A {
        match self.bits {
            false => SCPH_A::TOGGLE_MIDDLE,
            true => SCPH_A::TOGGLE_START,
        }
    }
    #[doc = "Checks if the value of the field is `TOGGLE_MIDDLE`"]
    #[inline(always)]
    pub fn is_toggle_middle(&self) -> bool {
        **self == SCPH_A::TOGGLE_MIDDLE
    }
    #[doc = "Checks if the value of the field is `TOGGLE_START`"]
    #[inline(always)]
    pub fn is_toggle_start(&self) -> bool {
        **self == SCPH_A::TOGGLE_START
    }
}
impl core::ops::Deref for SCPH_R {
    type Target = crate::FieldReader<bool, SCPH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCPH` writer - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH."]
pub struct SCPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCPH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCPH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Serial clock toggles in middle of first data bit"]
    #[inline(always)]
    pub fn toggle_middle(self) -> &'a mut W {
        self.variant(SCPH_A::TOGGLE_MIDDLE)
    }
    #[doc = "Serial clock toggles at start of first data bit"]
    #[inline(always)]
    pub fn toggle_start(self) -> &'a mut W {
        self.variant(SCPH_A::TOGGLE_START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI Master is not actively transferring data on the serial bus. Dependencies: When SSI_HC_FRF=1, SCPOL bit is a read-only bit with its value set by SSI_DFLT_SCPOL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCPOL_A {
    #[doc = "0: Inactive state of serial clock is low"]
    INACTIVE_LOW = 0,
    #[doc = "1: Inactive state of serial clock is high"]
    INACTIVE_HIGH = 1,
}
impl From<SCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCPOL` reader - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI Master is not actively transferring data on the serial bus. Dependencies: When SSI_HC_FRF=1, SCPOL bit is a read-only bit with its value set by SSI_DFLT_SCPOL."]
pub struct SCPOL_R(crate::FieldReader<bool, SCPOL_A>);
impl SCPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCPOL_A {
        match self.bits {
            false => SCPOL_A::INACTIVE_LOW,
            true => SCPOL_A::INACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_inactive_low(&self) -> bool {
        **self == SCPOL_A::INACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `INACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_inactive_high(&self) -> bool {
        **self == SCPOL_A::INACTIVE_HIGH
    }
}
impl core::ops::Deref for SCPOL_R {
    type Target = crate::FieldReader<bool, SCPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCPOL` writer - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI Master is not actively transferring data on the serial bus. Dependencies: When SSI_HC_FRF=1, SCPOL bit is a read-only bit with its value set by SSI_DFLT_SCPOL."]
pub struct SCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Inactive state of serial clock is low"]
    #[inline(always)]
    pub fn inactive_low(self) -> &'a mut W {
        self.variant(SCPOL_A::INACTIVE_LOW)
    }
    #[doc = "Inactive state of serial clock is high"]
    #[inline(always)]
    pub fn inactive_high(self) -> &'a mut W {
        self.variant(SCPOL_A::INACTIVE_HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Transfer Mode. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. In eeprom-read mode, receive data is not valid while control data is being transmitted. When all control data is sent to the EEPROM, receive data becomes valid and transmit data becomes invalid. All data in the transmit FIFO is considered control data in this mode. This transfer mode is only valid when the SPI Master is configured as a master device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMOD_A {
    #[doc = "0: Transmit and receive"]
    TX_RX = 0,
    #[doc = "1: Transmit only"]
    TX = 1,
    #[doc = "2: Receive only"]
    RX = 2,
    #[doc = "3: EEPROM read"]
    EEPROM = 3,
}
impl From<TMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMOD` reader - Transfer Mode. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. In eeprom-read mode, receive data is not valid while control data is being transmitted. When all control data is sent to the EEPROM, receive data becomes valid and transmit data becomes invalid. All data in the transmit FIFO is considered control data in this mode. This transfer mode is only valid when the SPI Master is configured as a master device."]
pub struct TMOD_R(crate::FieldReader<u8, TMOD_A>);
impl TMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOD_A {
        match self.bits {
            0 => TMOD_A::TX_RX,
            1 => TMOD_A::TX,
            2 => TMOD_A::RX,
            3 => TMOD_A::EEPROM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TX_RX`"]
    #[inline(always)]
    pub fn is_tx_rx(&self) -> bool {
        **self == TMOD_A::TX_RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        **self == TMOD_A::TX
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        **self == TMOD_A::RX
    }
    #[doc = "Checks if the value of the field is `EEPROM`"]
    #[inline(always)]
    pub fn is_eeprom(&self) -> bool {
        **self == TMOD_A::EEPROM
    }
}
impl core::ops::Deref for TMOD_R {
    type Target = crate::FieldReader<u8, TMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMOD` writer - Transfer Mode. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. In eeprom-read mode, receive data is not valid while control data is being transmitted. When all control data is sent to the EEPROM, receive data becomes valid and transmit data becomes invalid. All data in the transmit FIFO is considered control data in this mode. This transfer mode is only valid when the SPI Master is configured as a master device."]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Transmit and receive"]
    #[inline(always)]
    pub fn tx_rx(self) -> &'a mut W {
        self.variant(TMOD_A::TX_RX)
    }
    #[doc = "Transmit only"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(TMOD_A::TX)
    }
    #[doc = "Receive only"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(TMOD_A::RX)
    }
    #[doc = "EEPROM read"]
    #[inline(always)]
    pub fn eeprom(self) -> &'a mut W {
        self.variant(TMOD_A::EEPROM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SLV_OE` reader - No function for SPI Master. Slave usage only."]
pub struct SLV_OE_R(crate::FieldReader<bool, bool>);
impl SLV_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_OE` writer - No function for SPI Master. Slave usage only."]
pub struct SLV_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_OE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Shift Register Loop. Used for testing purposes only. When internally active, connects the transmit shift register output to the receive shift register input. Can be used in both serial slave and serial-master modes. When the SPI Master is configured as a slave in loopback mode, the ss_in_n and ssi_clk signals must be provided by an external source. In this mode, the slave cannot generate these signals because there is nothing to which to loop back.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRL_A {
    #[doc = "0: Normal mode operation"]
    NORMAL = 0,
    #[doc = "1: test mode operation"]
    TEST = 1,
}
impl From<SRL_A> for bool {
    #[inline(always)]
    fn from(variant: SRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRL` reader - Shift Register Loop. Used for testing purposes only. When internally active, connects the transmit shift register output to the receive shift register input. Can be used in both serial slave and serial-master modes. When the SPI Master is configured as a slave in loopback mode, the ss_in_n and ssi_clk signals must be provided by an external source. In this mode, the slave cannot generate these signals because there is nothing to which to loop back."]
pub struct SRL_R(crate::FieldReader<bool, SRL_A>);
impl SRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRL_A {
        match self.bits {
            false => SRL_A::NORMAL,
            true => SRL_A::TEST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SRL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        **self == SRL_A::TEST
    }
}
impl core::ops::Deref for SRL_R {
    type Target = crate::FieldReader<bool, SRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRL` writer - Shift Register Loop. Used for testing purposes only. When internally active, connects the transmit shift register output to the receive shift register input. Can be used in both serial slave and serial-master modes. When the SPI Master is configured as a slave in loopback mode, the ss_in_n and ssi_clk signals must be provided by an external source. In this mode, the slave cannot generate these signals because there is nothing to which to loop back."]
pub struct SRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SRL_A::NORMAL)
    }
    #[doc = "test mode operation"]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(SRL_A::TEST)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Control Frame Size. Selects the length of the control word for the Microwire frame format. For the field decode, refer to Table 6-3 on page 101\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFS_A {
    #[doc = "0: 1 bit control word"]
    WORD_1BIT = 0,
    #[doc = "1: 2 bit control word"]
    WORD_2BIT = 1,
    #[doc = "2: 3 bit control word"]
    WORD_3BIT = 2,
    #[doc = "3: 4 bit control word"]
    WORD_4BIT = 3,
    #[doc = "4: 5 bit control word"]
    WORD_5BIT = 4,
    #[doc = "5: 6 bit control word"]
    WORD_6BIT = 5,
    #[doc = "6: 7 bit control word"]
    WORD_7BIT = 6,
    #[doc = "7: 8 bit control word"]
    WORD_8BIT = 7,
    #[doc = "8: 9 bit control word"]
    WORD_9BIT = 8,
    #[doc = "9: 10 bit control word"]
    WORD_10BIT = 9,
    #[doc = "10: 11 bit control word"]
    WORD_11BIT = 10,
    #[doc = "11: 12 bit control word"]
    WORD_12BIT = 11,
    #[doc = "12: 13 bit control word"]
    WORD_13BIT = 12,
    #[doc = "13: 14 bit control word"]
    WORD_14BIT = 13,
    #[doc = "14: 15 bit control word"]
    WORD_15BIT = 14,
    #[doc = "15: 16 bit control word"]
    WORD_16BIT = 15,
}
impl From<CFS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFS` reader - Control Frame Size. Selects the length of the control word for the Microwire frame format. For the field decode, refer to Table 6-3 on page 101"]
pub struct CFS_R(crate::FieldReader<u8, CFS_A>);
impl CFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFS_A {
        match self.bits {
            0 => CFS_A::WORD_1BIT,
            1 => CFS_A::WORD_2BIT,
            2 => CFS_A::WORD_3BIT,
            3 => CFS_A::WORD_4BIT,
            4 => CFS_A::WORD_5BIT,
            5 => CFS_A::WORD_6BIT,
            6 => CFS_A::WORD_7BIT,
            7 => CFS_A::WORD_8BIT,
            8 => CFS_A::WORD_9BIT,
            9 => CFS_A::WORD_10BIT,
            10 => CFS_A::WORD_11BIT,
            11 => CFS_A::WORD_12BIT,
            12 => CFS_A::WORD_13BIT,
            13 => CFS_A::WORD_14BIT,
            14 => CFS_A::WORD_15BIT,
            15 => CFS_A::WORD_16BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WORD_1BIT`"]
    #[inline(always)]
    pub fn is_word_1bit(&self) -> bool {
        **self == CFS_A::WORD_1BIT
    }
    #[doc = "Checks if the value of the field is `WORD_2BIT`"]
    #[inline(always)]
    pub fn is_word_2bit(&self) -> bool {
        **self == CFS_A::WORD_2BIT
    }
    #[doc = "Checks if the value of the field is `WORD_3BIT`"]
    #[inline(always)]
    pub fn is_word_3bit(&self) -> bool {
        **self == CFS_A::WORD_3BIT
    }
    #[doc = "Checks if the value of the field is `WORD_4BIT`"]
    #[inline(always)]
    pub fn is_word_4bit(&self) -> bool {
        **self == CFS_A::WORD_4BIT
    }
    #[doc = "Checks if the value of the field is `WORD_5BIT`"]
    #[inline(always)]
    pub fn is_word_5bit(&self) -> bool {
        **self == CFS_A::WORD_5BIT
    }
    #[doc = "Checks if the value of the field is `WORD_6BIT`"]
    #[inline(always)]
    pub fn is_word_6bit(&self) -> bool {
        **self == CFS_A::WORD_6BIT
    }
    #[doc = "Checks if the value of the field is `WORD_7BIT`"]
    #[inline(always)]
    pub fn is_word_7bit(&self) -> bool {
        **self == CFS_A::WORD_7BIT
    }
    #[doc = "Checks if the value of the field is `WORD_8BIT`"]
    #[inline(always)]
    pub fn is_word_8bit(&self) -> bool {
        **self == CFS_A::WORD_8BIT
    }
    #[doc = "Checks if the value of the field is `WORD_9BIT`"]
    #[inline(always)]
    pub fn is_word_9bit(&self) -> bool {
        **self == CFS_A::WORD_9BIT
    }
    #[doc = "Checks if the value of the field is `WORD_10BIT`"]
    #[inline(always)]
    pub fn is_word_10bit(&self) -> bool {
        **self == CFS_A::WORD_10BIT
    }
    #[doc = "Checks if the value of the field is `WORD_11BIT`"]
    #[inline(always)]
    pub fn is_word_11bit(&self) -> bool {
        **self == CFS_A::WORD_11BIT
    }
    #[doc = "Checks if the value of the field is `WORD_12BIT`"]
    #[inline(always)]
    pub fn is_word_12bit(&self) -> bool {
        **self == CFS_A::WORD_12BIT
    }
    #[doc = "Checks if the value of the field is `WORD_13BIT`"]
    #[inline(always)]
    pub fn is_word_13bit(&self) -> bool {
        **self == CFS_A::WORD_13BIT
    }
    #[doc = "Checks if the value of the field is `WORD_14BIT`"]
    #[inline(always)]
    pub fn is_word_14bit(&self) -> bool {
        **self == CFS_A::WORD_14BIT
    }
    #[doc = "Checks if the value of the field is `WORD_15BIT`"]
    #[inline(always)]
    pub fn is_word_15bit(&self) -> bool {
        **self == CFS_A::WORD_15BIT
    }
    #[doc = "Checks if the value of the field is `WORD_16BIT`"]
    #[inline(always)]
    pub fn is_word_16bit(&self) -> bool {
        **self == CFS_A::WORD_16BIT
    }
}
impl core::ops::Deref for CFS_R {
    type Target = crate::FieldReader<u8, CFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFS` writer - Control Frame Size. Selects the length of the control word for the Microwire frame format. For the field decode, refer to Table 6-3 on page 101"]
pub struct CFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 bit control word"]
    #[inline(always)]
    pub fn word_1bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_1BIT)
    }
    #[doc = "2 bit control word"]
    #[inline(always)]
    pub fn word_2bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_2BIT)
    }
    #[doc = "3 bit control word"]
    #[inline(always)]
    pub fn word_3bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_3BIT)
    }
    #[doc = "4 bit control word"]
    #[inline(always)]
    pub fn word_4bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_4BIT)
    }
    #[doc = "5 bit control word"]
    #[inline(always)]
    pub fn word_5bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_5BIT)
    }
    #[doc = "6 bit control word"]
    #[inline(always)]
    pub fn word_6bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_6BIT)
    }
    #[doc = "7 bit control word"]
    #[inline(always)]
    pub fn word_7bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_7BIT)
    }
    #[doc = "8 bit control word"]
    #[inline(always)]
    pub fn word_8bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_8BIT)
    }
    #[doc = "9 bit control word"]
    #[inline(always)]
    pub fn word_9bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_9BIT)
    }
    #[doc = "10 bit control word"]
    #[inline(always)]
    pub fn word_10bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_10BIT)
    }
    #[doc = "11 bit control word"]
    #[inline(always)]
    pub fn word_11bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_11BIT)
    }
    #[doc = "12 bit control word"]
    #[inline(always)]
    pub fn word_12bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_12BIT)
    }
    #[doc = "13 bit control word"]
    #[inline(always)]
    pub fn word_13bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_13BIT)
    }
    #[doc = "14 bit control word"]
    #[inline(always)]
    pub fn word_14bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_14BIT)
    }
    #[doc = "15 bit control word"]
    #[inline(always)]
    pub fn word_15bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_15BIT)
    }
    #[doc = "16 bit control word"]
    #[inline(always)]
    pub fn word_16bit(self) -> &'a mut W {
        self.variant(CFS_A::WORD_16BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Data Frame Size in 32-bit mode. Used to select the data frame size in 32-bit mode. These bits are only valid when SSI_MAX_XFER_SIZE is configured to 32. When the data frame size is programmed to be less than 32 bits, the receive data is automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You are responsible for making sure the transmit data is right-justified before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode value, refer to Table 6-3 on page 101."]
pub type DFS_32_A = DFS_A;
#[doc = "Field `DFS_32` reader - Data Frame Size in 32-bit mode. Used to select the data frame size in 32-bit mode. These bits are only valid when SSI_MAX_XFER_SIZE is configured to 32. When the data frame size is programmed to be less than 32 bits, the receive data is automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You are responsible for making sure the transmit data is right-justified before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode value, refer to Table 6-3 on page 101."]
pub type DFS_32_R = DFS_R;
impl R {
    #[doc = "Bits 0:3 - Data Frame Size. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode, refer to Table 6-2."]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 00 - Motorola SPI only"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH."]
    #[inline(always)]
    pub fn scph(&self) -> SCPH_R {
        SCPH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI Master is not actively transferring data on the serial bus. Dependencies: When SSI_HC_FRF=1, SCPOL bit is a read-only bit with its value set by SSI_DFLT_SCPOL."]
    #[inline(always)]
    pub fn scpol(&self) -> SCPOL_R {
        SCPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Transfer Mode. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. In eeprom-read mode, receive data is not valid while control data is being transmitted. When all control data is sent to the EEPROM, receive data becomes valid and transmit data becomes invalid. All data in the transmit FIFO is considered control data in this mode. This transfer mode is only valid when the SPI Master is configured as a master device."]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - No function for SPI Master. Slave usage only."]
    #[inline(always)]
    pub fn slv_oe(&self) -> SLV_OE_R {
        SLV_OE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Shift Register Loop. Used for testing purposes only. When internally active, connects the transmit shift register output to the receive shift register input. Can be used in both serial slave and serial-master modes. When the SPI Master is configured as a slave in loopback mode, the ss_in_n and ssi_clk signals must be provided by an external source. In this mode, the slave cannot generate these signals because there is nothing to which to loop back."]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Control Frame Size. Selects the length of the control word for the Microwire frame format. For the field decode, refer to Table 6-3 on page 101"]
    #[inline(always)]
    pub fn cfs(&self) -> CFS_R {
        CFS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Data Frame Size in 32-bit mode. Used to select the data frame size in 32-bit mode. These bits are only valid when SSI_MAX_XFER_SIZE is configured to 32. When the data frame size is programmed to be less than 32 bits, the receive data is automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You are responsible for making sure the transmit data is right-justified before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode value, refer to Table 6-3 on page 101."]
    #[inline(always)]
    pub fn dfs_32(&self) -> DFS_32_R {
        DFS_32_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Frame Size. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH. Selects the data frame length. When the data frame size is programmed to be less than 16 bits, the receive data are automatically right-justified by the receive logic, with the upper bits of the receive FIFO zero-padded. You must right-justify transmit data before writing into the transmit FIFO. The transmit logic ignores the upper unused bits when transmitting the data. For the field decode, refer to Table 6-2."]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Bits 4:5 - 00 - Motorola SPI only"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bit 6 - Serial Clock Phase. Valid when the frame format (FRF) is set to Motorola SPI. The serial clock phase selects the relationship of the serial clock with the slave select signal. When SCPH = 0, data are captured on the first edge of the serial clock. When SCPH = 1, the serial clock starts toggling one cycle after the slave select line is activated, and data are captured on the second edge of the serial clock. Dependencies: When SSI_HC_FRF=1, SCPH bit is a read-only bit, with its value set by SSI_DFLT_SCPH."]
    #[inline(always)]
    pub fn scph(&mut self) -> SCPH_W {
        SCPH_W { w: self }
    }
    #[doc = "Bit 7 - Serial Clock Polarity. Valid when the frame format (FRF) is set to Motorola SPI. Used to select the polarity of the inactive serial clock, which is held inactive when the SPI Master is not actively transferring data on the serial bus. Dependencies: When SSI_HC_FRF=1, SCPOL bit is a read-only bit with its value set by SSI_DFLT_SCPOL."]
    #[inline(always)]
    pub fn scpol(&mut self) -> SCPOL_W {
        SCPOL_W { w: self }
    }
    #[doc = "Bits 8:9 - Transfer Mode. Selects the mode of transfer for serial communication. This field does not affect the transfer duplicity. Only indicates whether the receive or transmit data are valid. In transmit-only mode, data received from the external device is not valid and is not stored in the receive FIFO memory; it is overwritten on the next transfer. In receive-only mode, transmitted data are not valid. After the first write to the transmit FIFO, the same word is retransmitted for the duration of the transfer. In transmit-and-receive mode, both transmit and receive data are valid. The transfer continues until the transmit FIFO is empty. Data received from the external device are stored into the receive FIFO memory, where it can be accessed by the host processor. In eeprom-read mode, receive data is not valid while control data is being transmitted. When all control data is sent to the EEPROM, receive data becomes valid and transmit data becomes invalid. All data in the transmit FIFO is considered control data in this mode. This transfer mode is only valid when the SPI Master is configured as a master device."]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bit 10 - No function for SPI Master. Slave usage only."]
    #[inline(always)]
    pub fn slv_oe(&mut self) -> SLV_OE_W {
        SLV_OE_W { w: self }
    }
    #[doc = "Bit 11 - Shift Register Loop. Used for testing purposes only. When internally active, connects the transmit shift register output to the receive shift register input. Can be used in both serial slave and serial-master modes. When the SPI Master is configured as a slave in loopback mode, the ss_in_n and ssi_clk signals must be provided by an external source. In this mode, the slave cannot generate these signals because there is nothing to which to loop back."]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W {
        SRL_W { w: self }
    }
    #[doc = "Bits 12:15 - Control Frame Size. Selects the length of the control word for the Microwire frame format. For the field decode, refer to Table 6-3 on page 101"]
    #[inline(always)]
    pub fn cfs(&mut self) -> CFS_W {
        CFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 0: This register controls the serial data transfer. It is impossible to write to this register when the SPI Master is enabled. The SPI Master is enabled and disabled by writing to the SSIENR register (0x008).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlr0](index.html) module"]
pub struct CTRLR0_SPEC;
impl crate::RegisterSpec for CTRLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlr0::R](R) reader structure"]
impl crate::Readable for CTRLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlr0::W](W) writer structure"]
impl crate::Writable for CTRLR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLR0 to value 0"]
impl crate::Resettable for CTRLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
