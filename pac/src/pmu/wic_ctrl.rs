#[doc = "Register `WIC_CTRL` reader"]
pub struct R(crate::R<WIC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIC_CTRL` writer"]
pub struct W(crate::W<WIC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIC_CTRL_SPEC>;
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
impl From<crate::W<WIC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIC_Enable` reader - RWHC: This bit will be clear once M4 is in Mode 1 (SD mode) by HW."]
pub struct WIC_ENABLE_R(crate::FieldReader<bool, bool>);
impl WIC_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIC_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIC_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIC_Enable` writer - RWHC: This bit will be clear once M4 is in Mode 1 (SD mode) by HW."]
pub struct WIC_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIC_ENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RWHC: This bit will be clear once M4 is in Mode 1 (SD mode) by HW."]
    #[inline(always)]
    pub fn wic_enable(&self) -> WIC_ENABLE_R {
        WIC_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWHC: This bit will be clear once M4 is in Mode 1 (SD mode) by HW."]
    #[inline(always)]
    pub fn wic_enable(&mut self) -> WIC_ENABLE_W {
        WIC_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Interrupt Controller control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wic_ctrl](index.html) module"]
pub struct WIC_CTRL_SPEC;
impl crate::RegisterSpec for WIC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wic_ctrl::R](R) reader structure"]
impl crate::Readable for WIC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wic_ctrl::W](W) writer structure"]
impl crate::Writable for WIC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIC_CTRL to value 0"]
impl crate::Resettable for WIC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
