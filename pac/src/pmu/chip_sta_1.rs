#[doc = "Register `CHIP_STA_1` reader"]
pub struct R(crate::R<CHIP_STA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_STA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_STA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_STA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Indicates if FFE0 is busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE0_BUSY_A {
    #[doc = "0: FFE0 is not busy"]
    NOT_BUSY = 0,
    #[doc = "1: FFE0 is busy"]
    BUSY = 1,
}
impl From<FFE0_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: FFE0_BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE0_BUSY` reader - Indicates if FFE0 is busy"]
pub struct FFE0_BUSY_R(crate::FieldReader<bool, FFE0_BUSY_A>);
impl FFE0_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFE0_BUSY_A {
        match self.bits {
            false => FFE0_BUSY_A::NOT_BUSY,
            true => FFE0_BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        **self == FFE0_BUSY_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == FFE0_BUSY_A::BUSY
    }
}
impl core::ops::Deref for FFE0_BUSY_R {
    type Target = crate::FieldReader<bool, FFE0_BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This reflects the value of boot strap bit. Pad\\[19\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODE_SOURCE_CFG_A {
    #[doc = "0: From SPI Flash, if SWD_Mode_Cfg is 1, CfgSM will not be kicked off."]
    SPI_FLASH = 0,
    #[doc = "1: From AP"]
    AP = 1,
}
impl From<CODE_SOURCE_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: CODE_SOURCE_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Code_Source_Cfg` reader - This reflects the value of boot strap bit. Pad\\[19\\]"]
pub struct CODE_SOURCE_CFG_R(crate::FieldReader<bool, CODE_SOURCE_CFG_A>);
impl CODE_SOURCE_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CODE_SOURCE_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CODE_SOURCE_CFG_A {
        match self.bits {
            false => CODE_SOURCE_CFG_A::SPI_FLASH,
            true => CODE_SOURCE_CFG_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_FLASH`"]
    #[inline(always)]
    pub fn is_spi_flash(&self) -> bool {
        **self == CODE_SOURCE_CFG_A::SPI_FLASH
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        **self == CODE_SOURCE_CFG_A::AP
    }
}
impl core::ops::Deref for CODE_SOURCE_CFG_R {
    type Target = crate::FieldReader<bool, CODE_SOURCE_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This reflects the value of boot strap bit. Pad\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_PORT_CFG_A {
    #[doc = "0: SW_CLK @pad\\[5\\], SW_IO @pad\\[6\\]"]
    SWD_PADS_5_6 = 0,
    #[doc = "1: SW_CLK @pad\\[27\\], SW_IO @pad\\[26\\]"]
    SWD_PADS_27_26 = 1,
}
impl From<DEBUG_PORT_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_PORT_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Debug_Port_Cfg` reader - This reflects the value of boot strap bit. Pad\\[8\\]"]
pub struct DEBUG_PORT_CFG_R(crate::FieldReader<bool, DEBUG_PORT_CFG_A>);
impl DEBUG_PORT_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEBUG_PORT_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_PORT_CFG_A {
        match self.bits {
            false => DEBUG_PORT_CFG_A::SWD_PADS_5_6,
            true => DEBUG_PORT_CFG_A::SWD_PADS_27_26,
        }
    }
    #[doc = "Checks if the value of the field is `SWD_PADS_5_6`"]
    #[inline(always)]
    pub fn is_swd_pads_5_6(&self) -> bool {
        **self == DEBUG_PORT_CFG_A::SWD_PADS_5_6
    }
    #[doc = "Checks if the value of the field is `SWD_PADS_27_26`"]
    #[inline(always)]
    pub fn is_swd_pads_27_26(&self) -> bool {
        **self == DEBUG_PORT_CFG_A::SWD_PADS_27_26
    }
}
impl core::ops::Deref for DEBUG_PORT_CFG_R {
    type Target = crate::FieldReader<bool, DEBUG_PORT_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_0` reader - FB Stasus Siganl 0 (Its definition is depending on the implementation of FB)"]
pub struct FB_0_R(crate::FieldReader<bool, bool>);
impl FB_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_1` reader - FB Stasus Siganl 1 (Its definition is depending on the implementation of FB)"]
pub struct FB_1_R(crate::FieldReader<bool, bool>);
impl FB_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_2` reader - FB Stasus Siganl 2 (Its definition is depending on the implementation of FB)"]
pub struct FB_2_R(crate::FieldReader<bool, bool>);
impl FB_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_3` reader - FB Stasus Siganl 3 (Its definition is depending on the implementation of FB)"]
pub struct FB_3_R(crate::FieldReader<bool, bool>);
impl FB_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This reflects the value of boot strap bit. Pad\\[20\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWD_MODE_CFG_A {
    #[doc = "1: SWD debugger is attached. Release the M4 Core reset once System Reset released."]
    IF_SWD_RELEASE_M4_ON_REST = 1,
}
impl From<SWD_MODE_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: SWD_MODE_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWD_Mode_Cfg` reader - This reflects the value of boot strap bit. Pad\\[20\\]"]
pub struct SWD_MODE_CFG_R(crate::FieldReader<bool, SWD_MODE_CFG_A>);
impl SWD_MODE_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWD_MODE_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWD_MODE_CFG_A> {
        match self.bits {
            true => Some(SWD_MODE_CFG_A::IF_SWD_RELEASE_M4_ON_REST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IF_SWD_RELEASE_M4_ON_REST`"]
    #[inline(always)]
    pub fn is_if_swd_release_m4_on_rest(&self) -> bool {
        **self == SWD_MODE_CFG_A::IF_SWD_RELEASE_M4_ON_REST
    }
}
impl core::ops::Deref for SWD_MODE_CFG_R {
    type Target = crate::FieldReader<bool, SWD_MODE_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "This reflects the value of boot strap bit. Pad\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_BYPASS_CFG_A {
    #[doc = "1: The System Clock Source is from the I/O PAD instead of OSC if Debug_port_cfg is 1 as well."]
    EXTERNAL_CLOCK_CONFIGURED = 1,
}
impl From<CLOCK_BYPASS_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_BYPASS_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLOCK_BYPASS_Cfg` reader - This reflects the value of boot strap bit. Pad\\[9\\]"]
pub struct CLOCK_BYPASS_CFG_R(crate::FieldReader<bool, CLOCK_BYPASS_CFG_A>);
impl CLOCK_BYPASS_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_BYPASS_CFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCK_BYPASS_CFG_A> {
        match self.bits {
            true => Some(CLOCK_BYPASS_CFG_A::EXTERNAL_CLOCK_CONFIGURED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_CLOCK_CONFIGURED`"]
    #[inline(always)]
    pub fn is_external_clock_configured(&self) -> bool {
        **self == CLOCK_BYPASS_CFG_A::EXTERNAL_CLOCK_CONFIGURED
    }
}
impl core::ops::Deref for CLOCK_BYPASS_CFG_R {
    type Target = crate::FieldReader<bool, CLOCK_BYPASS_CFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if FFE0 is busy"]
    #[inline(always)]
    pub fn ffe0_busy(&self) -> FFE0_BUSY_R {
        FFE0_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - This reflects the value of boot strap bit. Pad\\[19\\]"]
    #[inline(always)]
    pub fn code_source_cfg(&self) -> CODE_SOURCE_CFG_R {
        CODE_SOURCE_CFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This reflects the value of boot strap bit. Pad\\[8\\]"]
    #[inline(always)]
    pub fn debug_port_cfg(&self) -> DEBUG_PORT_CFG_R {
        DEBUG_PORT_CFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FB Stasus Siganl 0 (Its definition is depending on the implementation of FB)"]
    #[inline(always)]
    pub fn fb_0(&self) -> FB_0_R {
        FB_0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FB Stasus Siganl 1 (Its definition is depending on the implementation of FB)"]
    #[inline(always)]
    pub fn fb_1(&self) -> FB_1_R {
        FB_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FB Stasus Siganl 2 (Its definition is depending on the implementation of FB)"]
    #[inline(always)]
    pub fn fb_2(&self) -> FB_2_R {
        FB_2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FB Stasus Siganl 3 (Its definition is depending on the implementation of FB)"]
    #[inline(always)]
    pub fn fb_3(&self) -> FB_3_R {
        FB_3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This reflects the value of boot strap bit. Pad\\[20\\]"]
    #[inline(always)]
    pub fn swd_mode_cfg(&self) -> SWD_MODE_CFG_R {
        SWD_MODE_CFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This reflects the value of boot strap bit. Pad\\[9\\]"]
    #[inline(always)]
    pub fn clock_bypass_cfg(&self) -> CLOCK_BYPASS_CFG_R {
        CLOCK_BYPASS_CFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
#[doc = "Chip Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_sta_1](index.html) module"]
pub struct CHIP_STA_1_SPEC;
impl crate::RegisterSpec for CHIP_STA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chip_sta_1::R](R) reader structure"]
impl crate::Readable for CHIP_STA_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHIP_STA_1 to value 0"]
impl crate::Resettable for CHIP_STA_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
