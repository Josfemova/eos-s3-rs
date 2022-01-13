#[doc = "Register `CONFIG_FP2` reader"]
pub struct R(crate::R<CONFIG_FP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_FP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_FP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_FP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_FP2` writer"]
pub struct W(crate::W<CONFIG_FP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_FP2_SPEC>;
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
impl From<crate::W<CONFIG_FP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_FP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIDZC_EN` reader - Mask exception to cpu; Floating-point divide-by-zero exception"]
pub struct FPIDZC_EN_R(crate::FieldReader<bool, bool>);
impl FPIDZC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIDZC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIDZC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIDZC_EN` writer - Mask exception to cpu; Floating-point divide-by-zero exception"]
pub struct FPIDZC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIDZC_EN_W<'a> {
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
#[doc = "Field `FPIOC_EN` reader - Mask exception to cpu; Floating-point invalid operation exception"]
pub struct FPIOC_EN_R(crate::FieldReader<bool, bool>);
impl FPIOC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIOC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIOC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIOC_EN` writer - Mask exception to cpu; Floating-point invalid operation exception"]
pub struct FPIOC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIOC_EN_W<'a> {
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
#[doc = "Field `FPIFC_EN` reader - Mask exception to cpu; Floating-point underflow exception"]
pub struct FPIFC_EN_R(crate::FieldReader<bool, bool>);
impl FPIFC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIFC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIFC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIFC_EN` writer - Mask exception to cpu; Floating-point underflow exception"]
pub struct FPIFC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIFC_EN_W<'a> {
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
#[doc = "Field `FPIDC_EN` reader - Mask exception to cpu; Floating-point overflow exception"]
pub struct FPIDC_EN_R(crate::FieldReader<bool, bool>);
impl FPIDC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIDC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIDC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIDC_EN` writer - Mask exception to cpu; Floating-point overflow exception"]
pub struct FPIDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIDC_EN_W<'a> {
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
#[doc = "Field `FPIXC_EN` reader - Mask exception to cpu; Floating-point inexact exception"]
pub struct FPIXC_EN_R(crate::FieldReader<bool, bool>);
impl FPIXC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPIXC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPIXC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPIXC_EN` writer - Mask exception to cpu; Floating-point inexact exception"]
pub struct FPIXC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIXC_EN_W<'a> {
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
impl R {
    #[doc = "Bit 1 - Mask exception to cpu; Floating-point divide-by-zero exception"]
    #[inline(always)]
    pub fn fpidzc_en(&self) -> FPIDZC_EN_R {
        FPIDZC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask exception to cpu; Floating-point invalid operation exception"]
    #[inline(always)]
    pub fn fpioc_en(&self) -> FPIOC_EN_R {
        FPIOC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask exception to cpu; Floating-point underflow exception"]
    #[inline(always)]
    pub fn fpifc_en(&self) -> FPIFC_EN_R {
        FPIFC_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask exception to cpu; Floating-point overflow exception"]
    #[inline(always)]
    pub fn fpidc_en(&self) -> FPIDC_EN_R {
        FPIDC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask exception to cpu; Floating-point inexact exception"]
    #[inline(always)]
    pub fn fpixc_en(&self) -> FPIXC_EN_R {
        FPIXC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mask exception to cpu; Floating-point divide-by-zero exception"]
    #[inline(always)]
    pub fn fpidzc_en(&mut self) -> FPIDZC_EN_W {
        FPIDZC_EN_W { w: self }
    }
    #[doc = "Bit 2 - Mask exception to cpu; Floating-point invalid operation exception"]
    #[inline(always)]
    pub fn fpioc_en(&mut self) -> FPIOC_EN_W {
        FPIOC_EN_W { w: self }
    }
    #[doc = "Bit 3 - Mask exception to cpu; Floating-point underflow exception"]
    #[inline(always)]
    pub fn fpifc_en(&mut self) -> FPIFC_EN_W {
        FPIFC_EN_W { w: self }
    }
    #[doc = "Bit 4 - Mask exception to cpu; Floating-point overflow exception"]
    #[inline(always)]
    pub fn fpidc_en(&mut self) -> FPIDC_EN_W {
        FPIDC_EN_W { w: self }
    }
    #[doc = "Bit 5 - Mask exception to cpu; Floating-point inexact exception"]
    #[inline(always)]
    pub fn fpixc_en(&mut self) -> FPIXC_EN_W {
        FPIXC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FPU configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_fp2](index.html) module"]
pub struct CONFIG_FP2_SPEC;
impl crate::RegisterSpec for CONFIG_FP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_fp2::R](R) reader structure"]
impl crate::Readable for CONFIG_FP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_fp2::W](W) writer structure"]
impl crate::Writable for CONFIG_FP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG_FP2 to value 0"]
impl crate::Resettable for CONFIG_FP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
