#[doc = "Register `PKFB_FIFOSRAMCTRL1` reader"]
pub struct R(crate::R<PKFB_FIFOSRAMCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_FIFOSRAMCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_FIFOSRAMCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_FIFOSRAMCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_FIFOSRAMCTRL1` writer"]
pub struct W(crate::W<PKFB_FIFOSRAMCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_FIFOSRAMCTRL1_SPEC>;
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
impl From<crate::W<PKFB_FIFOSRAMCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_FIFOSRAMCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf2_test1a` reader - Set this bit to disable test capabilities"]
pub struct PF2_TEST1A_R(crate::FieldReader<bool, bool>);
impl PF2_TEST1A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_TEST1A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_TEST1A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_test1a` writer - Set this bit to disable test capabilities"]
pub struct PF2_TEST1A_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_TEST1A_W<'a> {
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
#[doc = "Field `pf2_rmea` reader - Set this bit to disable SRAM timing adjust"]
pub struct PF2_RMEA_R(crate::FieldReader<bool, bool>);
impl PF2_RMEA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_RMEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_RMEA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_rmea` writer - Set this bit to disable SRAM timing adjust"]
pub struct PF2_RMEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_RMEA_W<'a> {
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
#[doc = "Field `pf2_rma` reader - SRAM Adjust timing value"]
pub struct PF2_RMA_R(crate::FieldReader<u8, u8>);
impl PF2_RMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF2_RMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_RMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_rma` writer - SRAM Adjust timing value"]
pub struct PF2_RMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_RMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `pf2_test1b` reader - Set this bit to disable test capabilities"]
pub struct PF2_TEST1B_R(crate::FieldReader<bool, bool>);
impl PF2_TEST1B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_TEST1B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_TEST1B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_test1b` writer - Set this bit to disable test capabilities"]
pub struct PF2_TEST1B_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_TEST1B_W<'a> {
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
#[doc = "Field `pf2_rmeb` reader - Set this bit to disable SRAM timing adjust"]
pub struct PF2_RMEB_R(crate::FieldReader<bool, bool>);
impl PF2_RMEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF2_RMEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_RMEB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_rmeb` writer - Set this bit to disable SRAM timing adjust"]
pub struct PF2_RMEB_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_RMEB_W<'a> {
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
#[doc = "Field `pf2_rmb` reader - SRAM Adjust timing value"]
pub struct PF2_RMB_R(crate::FieldReader<u8, u8>);
impl PF2_RMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF2_RMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF2_RMB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf2_rmb` writer - SRAM Adjust timing value"]
pub struct PF2_RMB_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_RMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `pf8k_test1a` reader - Set this bit to disable test capabilities"]
pub struct PF8K_TEST1A_R(crate::FieldReader<bool, bool>);
impl PF8K_TEST1A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_TEST1A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_TEST1A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_test1a` writer - Set this bit to disable test capabilities"]
pub struct PF8K_TEST1A_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_TEST1A_W<'a> {
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
#[doc = "Field `pf8k_rmea` reader - Set this bit to disable SRAM timing adjust"]
pub struct PF8K_RMEA_R(crate::FieldReader<bool, bool>);
impl PF8K_RMEA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_RMEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_RMEA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_rmea` writer - Set this bit to disable SRAM timing adjust"]
pub struct PF8K_RMEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_RMEA_W<'a> {
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
#[doc = "Field `pf8k_rma` reader - SRAM Adjust timing value"]
pub struct PF8K_RMA_R(crate::FieldReader<u8, u8>);
impl PF8K_RMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF8K_RMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_RMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_rma` writer - SRAM Adjust timing value"]
pub struct PF8K_RMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_RMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `pf8k_test1b` reader - Set this bit to disable test capabilities"]
pub struct PF8K_TEST1B_R(crate::FieldReader<bool, bool>);
impl PF8K_TEST1B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_TEST1B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_TEST1B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_test1b` writer - Set this bit to disable test capabilities"]
pub struct PF8K_TEST1B_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_TEST1B_W<'a> {
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
            (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `pf8k_rmeb` reader - Set this bit to disable SRAM timing adjust"]
pub struct PF8K_RMEB_R(crate::FieldReader<bool, bool>);
impl PF8K_RMEB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF8K_RMEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_RMEB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_rmeb` writer - Set this bit to disable SRAM timing adjust"]
pub struct PF8K_RMEB_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_RMEB_W<'a> {
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
            (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `pf8k_rmb` reader - SRAM Adjust timing value"]
pub struct PF8K_RMB_R(crate::FieldReader<u8, u8>);
impl PF8K_RMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF8K_RMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF8K_RMB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf8k_rmb` writer - SRAM Adjust timing value"]
pub struct PF8K_RMB_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_RMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 26)) | ((value as u32 & 0x0f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf2_test1a(&self) -> PF2_TEST1A_R {
        PF2_TEST1A_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf2_rmea(&self) -> PF2_RMEA_R {
        PF2_RMEA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf2_rma(&self) -> PF2_RMA_R {
        PF2_RMA_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf2_test1b(&self) -> PF2_TEST1B_R {
        PF2_TEST1B_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf2_rmeb(&self) -> PF2_RMEB_R {
        PF2_RMEB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf2_rmb(&self) -> PF2_RMB_R {
        PF2_RMB_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf8k_test1a(&self) -> PF8K_TEST1A_R {
        PF8K_TEST1A_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf8k_rmea(&self) -> PF8K_RMEA_R {
        PF8K_RMEA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf8k_rma(&self) -> PF8K_RMA_R {
        PF8K_RMA_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf8k_test1b(&self) -> PF8K_TEST1B_R {
        PF8K_TEST1B_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf8k_rmeb(&self) -> PF8K_RMEB_R {
        PF8K_RMEB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:29 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf8k_rmb(&self) -> PF8K_RMB_R {
        PF8K_RMB_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf2_test1a(&mut self) -> PF2_TEST1A_W {
        PF2_TEST1A_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf2_rmea(&mut self) -> PF2_RMEA_W {
        PF2_RMEA_W { w: self }
    }
    #[doc = "Bits 2:5 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf2_rma(&mut self) -> PF2_RMA_W {
        PF2_RMA_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf2_test1b(&mut self) -> PF2_TEST1B_W {
        PF2_TEST1B_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf2_rmeb(&mut self) -> PF2_RMEB_W {
        PF2_RMEB_W { w: self }
    }
    #[doc = "Bits 10:13 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf2_rmb(&mut self) -> PF2_RMB_W {
        PF2_RMB_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf8k_test1a(&mut self) -> PF8K_TEST1A_W {
        PF8K_TEST1A_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf8k_rmea(&mut self) -> PF8K_RMEA_W {
        PF8K_RMEA_W { w: self }
    }
    #[doc = "Bits 18:21 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf8k_rma(&mut self) -> PF8K_RMA_W {
        PF8K_RMA_W { w: self }
    }
    #[doc = "Bit 24 - Set this bit to disable test capabilities"]
    #[inline(always)]
    pub fn pf8k_test1b(&mut self) -> PF8K_TEST1B_W {
        PF8K_TEST1B_W { w: self }
    }
    #[doc = "Bit 25 - Set this bit to disable SRAM timing adjust"]
    #[inline(always)]
    pub fn pf8k_rmeb(&mut self) -> PF8K_RMEB_W {
        PF8K_RMEB_W { w: self }
    }
    #[doc = "Bits 26:29 - SRAM Adjust timing value"]
    #[inline(always)]
    pub fn pf8k_rmb(&mut self) -> PF8K_RMB_W {
        PF8K_RMB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Test Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_fifosramctrl1](index.html) module"]
pub struct PKFB_FIFOSRAMCTRL1_SPEC;
impl crate::RegisterSpec for PKFB_FIFOSRAMCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_fifosramctrl1::R](R) reader structure"]
impl crate::Readable for PKFB_FIFOSRAMCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_fifosramctrl1::W](W) writer structure"]
impl crate::Writable for PKFB_FIFOSRAMCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_FIFOSRAMCTRL1 to value 0"]
impl crate::Resettable for PKFB_FIFOSRAMCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
