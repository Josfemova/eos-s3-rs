#[doc = "Register `SRAM_TEST_REG2` reader"]
pub struct R(crate::R<SRAM_TEST_REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_TEST_REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_TEST_REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_TEST_REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_TEST_REG2` writer"]
pub struct W(crate::W<SRAM_TEST_REG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_TEST_REG2_SPEC>;
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
impl From<crate::W<SRAM_TEST_REG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_TEST_REG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DM1_RM` reader - DM0_RM control for FFE SRAM"]
pub struct DM1_RM_R(crate::FieldReader<u8, u8>);
impl DM1_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM1_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM1_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM1_RM` writer - DM0_RM control for FFE SRAM"]
pub struct DM1_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DM2_TEST1` reader - DM2_TEST1 control for FFE SRAM"]
pub struct DM2_TEST1_R(crate::FieldReader<bool, bool>);
impl DM2_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM2_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM2_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM2_TEST1` writer - DM2_TEST1 control for FFE SRAM"]
pub struct DM2_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> DM2_TEST1_W<'a> {
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
#[doc = "Field `DM2_RME` reader - DM2_RME control for FFE SRAM"]
pub struct DM2_RME_R(crate::FieldReader<bool, bool>);
impl DM2_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM2_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM2_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM2_RME` writer - DM2_RME control for FFE SRAM"]
pub struct DM2_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> DM2_RME_W<'a> {
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
#[doc = "Field `DM2_RM` reader - DM0_RM control for FFE SRAM"]
pub struct DM2_RM_R(crate::FieldReader<u8, u8>);
impl DM2_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM2_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM2_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM2_RM` writer - DM0_RM control for FFE SRAM"]
pub struct DM2_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM2_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Field `DM3_TEST1` reader - DM3_TEST1 control for FFE SRAM"]
pub struct DM3_TEST1_R(crate::FieldReader<bool, bool>);
impl DM3_TEST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM3_TEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM3_TEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM3_TEST1` writer - DM3_TEST1 control for FFE SRAM"]
pub struct DM3_TEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> DM3_TEST1_W<'a> {
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
#[doc = "Field `DM3_RME` reader - DM3_RME control for FFE SRAM"]
pub struct DM3_RME_R(crate::FieldReader<bool, bool>);
impl DM3_RME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DM3_RME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM3_RME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM3_RME` writer - DM3_RME control for FFE SRAM"]
pub struct DM3_RME_W<'a> {
    w: &'a mut W,
}
impl<'a> DM3_RME_W<'a> {
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
#[doc = "Field `DM3_RM` reader - DM3_RM control for FFE SRAM"]
pub struct DM3_RM_R(crate::FieldReader<u8, u8>);
impl DM3_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM3_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM3_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM3_RM` writer - DM3_RM control for FFE SRAM"]
pub struct DM3_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM3_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm1_rm(&self) -> DM1_RM_R {
        DM1_RM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DM2_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm2_test1(&self) -> DM2_TEST1_R {
        DM2_TEST1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DM2_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm2_rme(&self) -> DM2_RME_R {
        DM2_RME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - DM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm2_rm(&self) -> DM2_RM_R {
        DM2_RM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - DM3_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm3_test1(&self) -> DM3_TEST1_R {
        DM3_TEST1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DM3_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm3_rme(&self) -> DM3_RME_R {
        DM3_RME_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - DM3_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm3_rm(&self) -> DM3_RM_R {
        DM3_RM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm1_rm(&mut self) -> DM1_RM_W {
        DM1_RM_W { w: self }
    }
    #[doc = "Bit 4 - DM2_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm2_test1(&mut self) -> DM2_TEST1_W {
        DM2_TEST1_W { w: self }
    }
    #[doc = "Bit 5 - DM2_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm2_rme(&mut self) -> DM2_RME_W {
        DM2_RME_W { w: self }
    }
    #[doc = "Bits 6:9 - DM0_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm2_rm(&mut self) -> DM2_RM_W {
        DM2_RM_W { w: self }
    }
    #[doc = "Bit 10 - DM3_TEST1 control for FFE SRAM"]
    #[inline(always)]
    pub fn dm3_test1(&mut self) -> DM3_TEST1_W {
        DM3_TEST1_W { w: self }
    }
    #[doc = "Bit 11 - DM3_RME control for FFE SRAM"]
    #[inline(always)]
    pub fn dm3_rme(&mut self) -> DM3_RME_W {
        DM3_RME_W { w: self }
    }
    #[doc = "Bits 12:15 - DM3_RM control for FFE SRAM"]
    #[inline(always)]
    pub fn dm3_rm(&mut self) -> DM3_RM_W {
        DM3_RM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM test control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_test_reg2](index.html) module"]
pub struct SRAM_TEST_REG2_SPEC;
impl crate::RegisterSpec for SRAM_TEST_REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_test_reg2::R](R) reader structure"]
impl crate::Readable for SRAM_TEST_REG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_test_reg2::W](W) writer structure"]
impl crate::Writable for SRAM_TEST_REG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_TEST_REG2 to value 0"]
impl crate::Resettable for SRAM_TEST_REG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
