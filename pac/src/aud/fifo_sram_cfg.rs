#[doc = "Register `FIFO_SRAM_CFG` reader"]
pub struct R(crate::R<FIFO_SRAM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SRAM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SRAM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SRAM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_SRAM_CFG` writer"]
pub struct W(crate::W<FIFO_SRAM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SRAM_CFG_SPEC>;
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
impl From<crate::W<FIFO_SRAM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SRAM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_0A_TEST1` reader - Test pin to bypass self-timed circuit"]
pub struct SRAM_0A_TEST1_R(crate::FieldReader<bool, bool>);
impl SRAM_0A_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_0A_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_0A_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_0A_TEST1` writer - Test pin to bypass self-timed circuit"]
pub struct SRAM_0A_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_0A_TEST1_W<'a> {
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
#[doc = "Field `SRAM_0A_RME` reader - Read-Write margin Enable input"]
pub struct SRAM_0A_RME_R(crate::FieldReader<bool, bool>);
impl SRAM_0A_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_0A_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_0A_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_0A_RME` writer - Read-Write margin Enable input"]
pub struct SRAM_0A_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_0A_RME_W<'a> {
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
#[doc = "Field `SRAM_0A_RM` reader - Read-Write margin Input for Left Channel 8KB FIFO"]
pub struct SRAM_0A_RM_R(crate::FieldReader<u8, u8>);
impl SRAM_0A_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_0A_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_0A_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_0A_RM` writer - Read-Write margin Input for Left Channel 8KB FIFO"]
pub struct SRAM_0A_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_0A_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `SRAM_0B_TEST1` reader - Test pin to bypass self-timed circuit"]
pub struct SRAM_0B_TEST1_R(crate::FieldReader<bool, bool>);
impl SRAM_0B_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_0B_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_0B_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_0B_TEST1` writer - Test pin to bypass self-timed circuit"]
pub struct SRAM_0B_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_0B_TEST1_W<'a> {
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
#[doc = "Field `SRAM_0B_RME` reader - Read-Write margin Enable input"]
pub struct SRAM_0B_RME_R(crate::FieldReader<bool, bool>);
impl SRAM_0B_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_0B_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_0B_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_0B_RME` writer - Read-Write margin Enable input"]
pub struct SRAM_0B_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_0B_RME_W<'a> {
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
#[doc = "Field `SRAM_0B_RM` reader - Read-Write margin Input for Left Channel 512B FIFO"]
pub struct SRAM_0B_RM_R(crate::FieldReader<u8, u8>);
impl SRAM_0B_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_0B_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_0B_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_0B_RM` writer - Read-Write margin Input for Left Channel 512B FIFO"]
pub struct SRAM_0B_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_0B_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SRAM_1A_TEST1` reader - Test pin to bypass self-timed circuit"]
pub struct SRAM_1A_TEST1_R(crate::FieldReader<bool, bool>);
impl SRAM_1A_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_1A_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_1A_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_1A_TEST1` writer - Test pin to bypass self-timed circuit"]
pub struct SRAM_1A_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_1A_TEST1_W<'a> {
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
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SRAM_1A_RME` reader - Read-Write margin Enable input"]
pub struct SRAM_1A_RME_R(crate::FieldReader<bool, bool>);
impl SRAM_1A_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_1A_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_1A_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_1A_RME` writer - Read-Write margin Enable input"]
pub struct SRAM_1A_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_1A_RME_W<'a> {
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
            (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `SRAM_1A_RM` reader - Read-Write margin Input for Right Channel 8KB FIFO"]
pub struct SRAM_1A_RM_R(crate::FieldReader<u8, u8>);
impl SRAM_1A_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_1A_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_1A_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_1A_RM` writer - Read-Write margin Input for Right Channel 8KB FIFO"]
pub struct SRAM_1A_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_1A_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `SRAM_1B_TEST1` reader - Test pin to bypass self-timed circuit"]
pub struct SRAM_1B_TEST1_R(crate::FieldReader<bool, bool>);
impl SRAM_1B_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_1B_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_1B_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_1B_TEST1` writer - Test pin to bypass self-timed circuit"]
pub struct SRAM_1B_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_1B_TEST1_W<'a> {
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
#[doc = "Field `SRAM_1B_RME` reader - Read-Write margin Enable input"]
pub struct SRAM_1B_RME_R(crate::FieldReader<bool, bool>);
impl SRAM_1B_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_1B_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_1B_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_1B_RME` writer - Read-Write margin Enable input"]
pub struct SRAM_1B_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_1B_RME_W<'a> {
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
            (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SRAM_1B_RM` reader - Read-Write margin Input for Right Channel 512B FIFO"]
pub struct SRAM_1B_RM_R(crate::FieldReader<u8, u8>);
impl SRAM_1B_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRAM_1B_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_1B_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_1B_RM` writer - Read-Write margin Input for Right Channel 512B FIFO"]
pub struct SRAM_1B_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_1B_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_0a_test1(&self) -> SRAM_0A_TEST1_R {
        SRAM_0A_TEST1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_0a_rme(&self) -> SRAM_0A_RME_R {
        SRAM_0A_RME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Read-Write margin Input for Left Channel 8KB FIFO"]
    #[inline(always)]
    pub fn sram_0a_rm(&self) -> SRAM_0A_RM_R {
        SRAM_0A_RM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_0b_test1(&self) -> SRAM_0B_TEST1_R {
        SRAM_0B_TEST1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_0b_rme(&self) -> SRAM_0B_RME_R {
        SRAM_0B_RME_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Read-Write margin Input for Left Channel 512B FIFO"]
    #[inline(always)]
    pub fn sram_0b_rm(&self) -> SRAM_0B_RM_R {
        SRAM_0B_RM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_1a_test1(&self) -> SRAM_1A_TEST1_R {
        SRAM_1A_TEST1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_1a_rme(&self) -> SRAM_1A_RME_R {
        SRAM_1A_RME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:17 - Read-Write margin Input for Right Channel 8KB FIFO"]
    #[inline(always)]
    pub fn sram_1a_rm(&self) -> SRAM_1A_RM_R {
        SRAM_1A_RM_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_1b_test1(&self) -> SRAM_1B_TEST1_R {
        SRAM_1B_TEST1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_1b_rme(&self) -> SRAM_1B_RME_R {
        SRAM_1B_RME_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Read-Write margin Input for Right Channel 512B FIFO"]
    #[inline(always)]
    pub fn sram_1b_rm(&self) -> SRAM_1B_RM_R {
        SRAM_1B_RM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_0a_test1(&mut self) -> SRAM_0A_TEST1_W {
        SRAM_0A_TEST1_W { w: self }
    }
    #[doc = "Bit 1 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_0a_rme(&mut self) -> SRAM_0A_RME_W {
        SRAM_0A_RME_W { w: self }
    }
    #[doc = "Bits 2:5 - Read-Write margin Input for Left Channel 8KB FIFO"]
    #[inline(always)]
    pub fn sram_0a_rm(&mut self) -> SRAM_0A_RM_W {
        SRAM_0A_RM_W { w: self }
    }
    #[doc = "Bit 6 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_0b_test1(&mut self) -> SRAM_0B_TEST1_W {
        SRAM_0B_TEST1_W { w: self }
    }
    #[doc = "Bit 7 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_0b_rme(&mut self) -> SRAM_0B_RME_W {
        SRAM_0B_RME_W { w: self }
    }
    #[doc = "Bits 8:11 - Read-Write margin Input for Left Channel 512B FIFO"]
    #[inline(always)]
    pub fn sram_0b_rm(&mut self) -> SRAM_0B_RM_W {
        SRAM_0B_RM_W { w: self }
    }
    #[doc = "Bit 12 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_1a_test1(&mut self) -> SRAM_1A_TEST1_W {
        SRAM_1A_TEST1_W { w: self }
    }
    #[doc = "Bit 13 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_1a_rme(&mut self) -> SRAM_1A_RME_W {
        SRAM_1A_RME_W { w: self }
    }
    #[doc = "Bits 14:17 - Read-Write margin Input for Right Channel 8KB FIFO"]
    #[inline(always)]
    pub fn sram_1a_rm(&mut self) -> SRAM_1A_RM_W {
        SRAM_1A_RM_W { w: self }
    }
    #[doc = "Bit 18 - Test pin to bypass self-timed circuit"]
    #[inline(always)]
    pub fn sram_1b_test1(&mut self) -> SRAM_1B_TEST1_W {
        SRAM_1B_TEST1_W { w: self }
    }
    #[doc = "Bit 19 - Read-Write margin Enable input"]
    #[inline(always)]
    pub fn sram_1b_rme(&mut self) -> SRAM_1B_RME_W {
        SRAM_1B_RME_W { w: self }
    }
    #[doc = "Bits 20:23 - Read-Write margin Input for Right Channel 512B FIFO"]
    #[inline(always)]
    pub fn sram_1b_rm(&mut self) -> SRAM_1B_RM_W {
        SRAM_1B_RM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO SRAM configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_sram_cfg](index.html) module"]
pub struct FIFO_SRAM_CFG_SPEC;
impl crate::RegisterSpec for FIFO_SRAM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_sram_cfg::R](R) reader structure"]
impl crate::Readable for FIFO_SRAM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_sram_cfg::W](W) writer structure"]
impl crate::Writable for FIFO_SRAM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_SRAM_CFG to value 0"]
impl crate::Resettable for FIFO_SRAM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
