#[doc = "Register `MEM_PWR_DWN_CTRL` reader"]
pub struct R(crate::R<MEM_PWR_DWN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_PWR_DWN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_PWR_DWN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_PWR_DWN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_PWR_DWN_CTRL` writer"]
pub struct W(crate::W<MEM_PWR_DWN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_PWR_DWN_CTRL_SPEC>;
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
impl From<crate::W<MEM_PWR_DWN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_PWR_DWN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S0` reader - Set to put M4S0 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S0_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S0` writer - Set to put M4S0 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S0_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S1` reader - Set to put M4S1 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S1_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S1` writer - Set to put M4S1 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S1_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S1_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S2` reader - Set to put M4S2 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S2_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S2` writer - Set to put M4S2 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S2_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S2_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S3` reader - Set to put M4S3 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S3_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S3` writer - Set to put M4S3 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S3_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S3_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S4` reader - Set to put M4S4 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S4_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S4` writer - Set to put M4S4 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S4_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S4_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S5` reader - Set to put M4S5 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S5_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S5` writer - Set to put M4S5 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S5_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S5_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S6` reader - Set to put M4S6 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S6_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S6` writer - Set to put M4S6 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S6_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S6_W<'a> {
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
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S7` reader - Set to put M4S7 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S7_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S7` writer - Set to put M4S7 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S7_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S7_W<'a> {
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
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S8` reader - Set to put M4S8 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S8_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S8` writer - Set to put M4S8 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S8_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S8_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S9` reader - Set to put M4S9 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S9_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S9` writer - Set to put M4S9 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S9_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S9_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S10` reader - Set tout M4S10 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S10_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S10` writer - Set tout M4S10 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S10_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S10_W<'a> {
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
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S11` reader - Set to put M4S11 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S11_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_0_M4S11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_0_M4S11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_0_M4S11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_0_M4S11` writer - Set to put M4S11 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_0_M4S11_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_PD_CFG_0_M4S11_W<'a> {
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
#[doc = "Field `M4_SRAM_PD_Cfg_1_M4S12` reader - Set to put M4S12 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_1_M4S12_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_1_M4S12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_1_M4S12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_1_M4S12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_1_M4S13` reader - Set to put M4S13 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_1_M4S13_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_1_M4S13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_1_M4S13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_1_M4S13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_1_M4S14` reader - Set to put M4S14 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_1_M4S14_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_1_M4S14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_1_M4S14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_1_M4S14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_PD_Cfg_1_M4S15` reader - Set to put M4S15 to Deep Sleep mode if M4 is in Shut Down Mode"]
pub struct M4_SRAM_PD_CFG_1_M4S15_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_PD_CFG_1_M4S15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_PD_CFG_1_M4S15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_PD_CFG_1_M4S15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_SRAM_PD_Cfg` reader - Set to assert the DS pin of the SRAM Macro inside FFE power domain if FFE power domain is in Deep Sleep"]
pub struct FFE_SRAM_PD_CFG_R(crate::FieldReader<bool, bool>);
impl FFE_SRAM_PD_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE_SRAM_PD_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_SRAM_PD_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_SRAM_PD_Cfg` writer - Set to assert the DS pin of the SRAM Macro inside FFE power domain if FFE power domain is in Deep Sleep"]
pub struct FFE_SRAM_PD_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_SRAM_PD_CFG_W<'a> {
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
            (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PF_SRAM_PD_Cfg` reader - Set to assert the DS pin of the SRAM Macro inside PF power domain if PF power domain is in Deep Sleep"]
pub struct PF_SRAM_PD_CFG_R(crate::FieldReader<bool, bool>);
impl PF_SRAM_PD_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF_SRAM_PD_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF_SRAM_PD_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF_SRAM_PD_Cfg` writer - Set to assert the DS pin of the SRAM Macro inside PF power domain if PF power domain is in Deep Sleep"]
pub struct PF_SRAM_PD_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_SRAM_PD_CFG_W<'a> {
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
            (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SDMA_SRAM_PD_Cfg` reader - Set to assert the DS pin of the SRAM Macro inside SDMA power domain if SDMA power domain is in Deep Sleep or Shut Down Mode."]
pub struct SDMA_SRAM_PD_CFG_R(crate::FieldReader<bool, bool>);
impl SDMA_SRAM_PD_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMA_SRAM_PD_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMA_SRAM_PD_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA_SRAM_PD_Cfg` writer - Set to assert the DS pin of the SRAM Macro inside SDMA power domain if SDMA power domain is in Deep Sleep or Shut Down Mode."]
pub struct SDMA_SRAM_PD_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA_SRAM_PD_CFG_W<'a> {
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
            (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to put M4S0 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s0(&self) -> M4_SRAM_PD_CFG_0_M4S0_R {
        M4_SRAM_PD_CFG_0_M4S0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to put M4S1 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s1(&self) -> M4_SRAM_PD_CFG_0_M4S1_R {
        M4_SRAM_PD_CFG_0_M4S1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to put M4S2 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s2(&self) -> M4_SRAM_PD_CFG_0_M4S2_R {
        M4_SRAM_PD_CFG_0_M4S2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to put M4S3 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s3(&self) -> M4_SRAM_PD_CFG_0_M4S3_R {
        M4_SRAM_PD_CFG_0_M4S3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to put M4S4 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s4(&self) -> M4_SRAM_PD_CFG_0_M4S4_R {
        M4_SRAM_PD_CFG_0_M4S4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set to put M4S5 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s5(&self) -> M4_SRAM_PD_CFG_0_M4S5_R {
        M4_SRAM_PD_CFG_0_M4S5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set to put M4S6 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s6(&self) -> M4_SRAM_PD_CFG_0_M4S6_R {
        M4_SRAM_PD_CFG_0_M4S6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set to put M4S7 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s7(&self) -> M4_SRAM_PD_CFG_0_M4S7_R {
        M4_SRAM_PD_CFG_0_M4S7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set to put M4S8 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s8(&self) -> M4_SRAM_PD_CFG_0_M4S8_R {
        M4_SRAM_PD_CFG_0_M4S8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set to put M4S9 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s9(&self) -> M4_SRAM_PD_CFG_0_M4S9_R {
        M4_SRAM_PD_CFG_0_M4S9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Set tout M4S10 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s10(&self) -> M4_SRAM_PD_CFG_0_M4S10_R {
        M4_SRAM_PD_CFG_0_M4S10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Set to put M4S11 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s11(&self) -> M4_SRAM_PD_CFG_0_M4S11_R {
        M4_SRAM_PD_CFG_0_M4S11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Set to put M4S12 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_1_m4s12(&self) -> M4_SRAM_PD_CFG_1_M4S12_R {
        M4_SRAM_PD_CFG_1_M4S12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set to put M4S13 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_1_m4s13(&self) -> M4_SRAM_PD_CFG_1_M4S13_R {
        M4_SRAM_PD_CFG_1_M4S13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set to put M4S14 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_1_m4s14(&self) -> M4_SRAM_PD_CFG_1_M4S14_R {
        M4_SRAM_PD_CFG_1_M4S14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Set to put M4S15 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_1_m4s15(&self) -> M4_SRAM_PD_CFG_1_M4S15_R {
        M4_SRAM_PD_CFG_1_M4S15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set to assert the DS pin of the SRAM Macro inside FFE power domain if FFE power domain is in Deep Sleep"]
    #[inline(always)]
    pub fn ffe_sram_pd_cfg(&self) -> FFE_SRAM_PD_CFG_R {
        FFE_SRAM_PD_CFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set to assert the DS pin of the SRAM Macro inside PF power domain if PF power domain is in Deep Sleep"]
    #[inline(always)]
    pub fn pf_sram_pd_cfg(&self) -> PF_SRAM_PD_CFG_R {
        PF_SRAM_PD_CFG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Set to assert the DS pin of the SRAM Macro inside SDMA power domain if SDMA power domain is in Deep Sleep or Shut Down Mode."]
    #[inline(always)]
    pub fn sdma_sram_pd_cfg(&self) -> SDMA_SRAM_PD_CFG_R {
        SDMA_SRAM_PD_CFG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to put M4S0 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s0(&mut self) -> M4_SRAM_PD_CFG_0_M4S0_W {
        M4_SRAM_PD_CFG_0_M4S0_W { w: self }
    }
    #[doc = "Bit 1 - Set to put M4S1 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s1(&mut self) -> M4_SRAM_PD_CFG_0_M4S1_W {
        M4_SRAM_PD_CFG_0_M4S1_W { w: self }
    }
    #[doc = "Bit 2 - Set to put M4S2 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s2(&mut self) -> M4_SRAM_PD_CFG_0_M4S2_W {
        M4_SRAM_PD_CFG_0_M4S2_W { w: self }
    }
    #[doc = "Bit 3 - Set to put M4S3 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s3(&mut self) -> M4_SRAM_PD_CFG_0_M4S3_W {
        M4_SRAM_PD_CFG_0_M4S3_W { w: self }
    }
    #[doc = "Bit 4 - Set to put M4S4 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s4(&mut self) -> M4_SRAM_PD_CFG_0_M4S4_W {
        M4_SRAM_PD_CFG_0_M4S4_W { w: self }
    }
    #[doc = "Bit 5 - Set to put M4S5 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s5(&mut self) -> M4_SRAM_PD_CFG_0_M4S5_W {
        M4_SRAM_PD_CFG_0_M4S5_W { w: self }
    }
    #[doc = "Bit 6 - Set to put M4S6 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s6(&mut self) -> M4_SRAM_PD_CFG_0_M4S6_W {
        M4_SRAM_PD_CFG_0_M4S6_W { w: self }
    }
    #[doc = "Bit 7 - Set to put M4S7 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s7(&mut self) -> M4_SRAM_PD_CFG_0_M4S7_W {
        M4_SRAM_PD_CFG_0_M4S7_W { w: self }
    }
    #[doc = "Bit 8 - Set to put M4S8 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s8(&mut self) -> M4_SRAM_PD_CFG_0_M4S8_W {
        M4_SRAM_PD_CFG_0_M4S8_W { w: self }
    }
    #[doc = "Bit 9 - Set to put M4S9 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s9(&mut self) -> M4_SRAM_PD_CFG_0_M4S9_W {
        M4_SRAM_PD_CFG_0_M4S9_W { w: self }
    }
    #[doc = "Bit 10 - Set tout M4S10 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s10(&mut self) -> M4_SRAM_PD_CFG_0_M4S10_W {
        M4_SRAM_PD_CFG_0_M4S10_W { w: self }
    }
    #[doc = "Bit 11 - Set to put M4S11 to Deep Sleep mode if M4 is in Shut Down Mode"]
    #[inline(always)]
    pub fn m4_sram_pd_cfg_0_m4s11(&mut self) -> M4_SRAM_PD_CFG_0_M4S11_W {
        M4_SRAM_PD_CFG_0_M4S11_W { w: self }
    }
    #[doc = "Bit 16 - Set to assert the DS pin of the SRAM Macro inside FFE power domain if FFE power domain is in Deep Sleep"]
    #[inline(always)]
    pub fn ffe_sram_pd_cfg(&mut self) -> FFE_SRAM_PD_CFG_W {
        FFE_SRAM_PD_CFG_W { w: self }
    }
    #[doc = "Bit 17 - Set to assert the DS pin of the SRAM Macro inside PF power domain if PF power domain is in Deep Sleep"]
    #[inline(always)]
    pub fn pf_sram_pd_cfg(&mut self) -> PF_SRAM_PD_CFG_W {
        PF_SRAM_PD_CFG_W { w: self }
    }
    #[doc = "Bit 18 - Set to assert the DS pin of the SRAM Macro inside SDMA power domain if SDMA power domain is in Deep Sleep or Shut Down Mode."]
    #[inline(always)]
    pub fn sdma_sram_pd_cfg(&mut self) -> SDMA_SRAM_PD_CFG_W {
        SDMA_SRAM_PD_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Power Down Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pwr_dwn_ctrl](index.html) module"]
pub struct MEM_PWR_DWN_CTRL_SPEC;
impl crate::RegisterSpec for MEM_PWR_DWN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_pwr_dwn_ctrl::R](R) reader structure"]
impl crate::Readable for MEM_PWR_DWN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_pwr_dwn_ctrl::W](W) writer structure"]
impl crate::Writable for MEM_PWR_DWN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_PWR_DWN_CTRL to value 0"]
impl crate::Resettable for MEM_PWR_DWN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
