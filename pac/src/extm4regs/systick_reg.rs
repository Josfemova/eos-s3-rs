#[doc = "Register `SYSTICK_REG` reader"]
pub struct R(crate::R<SYSTICK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTICK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTICK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTICK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTICK_REG` writer"]
pub struct W(crate::W<SYSTICK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTICK_REG_SPEC>;
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
impl From<crate::W<SYSTICK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTICK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTICK_TENMS` reader - Provides an integer value to compute a 10ms (100Hz) delay. For example, apply the value 0x07A11F if no reference is implemented, and FCLK is 50MHz."]
pub struct SYSTICK_TENMS_R(crate::FieldReader<u32, u32>);
impl SYSTICK_TENMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SYSTICK_TENMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTICK_TENMS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTICK_TENMS` writer - Provides an integer value to compute a 10ms (100Hz) delay. For example, apply the value 0x07A11F if no reference is implemented, and FCLK is 50MHz."]
pub struct SYSTICK_TENMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICK_TENMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `SYSTICK_SKEW` reader - Systick clock is an exact multiple of 10ms"]
pub struct SYSTICK_SKEW_R(crate::FieldReader<bool, bool>);
impl SYSTICK_SKEW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSTICK_SKEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTICK_SKEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTICK_SKEW` writer - Systick clock is an exact multiple of 10ms"]
pub struct SYSTICK_SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICK_SKEW_W<'a> {
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
#[doc = "Field `SYSTICK_NOREF` reader - Indicates that no alternative reference clock source has been integrated."]
pub struct SYSTICK_NOREF_R(crate::FieldReader<bool, bool>);
impl SYSTICK_NOREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSTICK_NOREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTICK_NOREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTICK_NOREF` writer - Indicates that no alternative reference clock source has been integrated."]
pub struct SYSTICK_NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICK_NOREF_W<'a> {
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
impl R {
    #[doc = "Bits 0:23 - Provides an integer value to compute a 10ms (100Hz) delay. For example, apply the value 0x07A11F if no reference is implemented, and FCLK is 50MHz."]
    #[inline(always)]
    pub fn systick_tenms(&self) -> SYSTICK_TENMS_R {
        SYSTICK_TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Systick clock is an exact multiple of 10ms"]
    #[inline(always)]
    pub fn systick_skew(&self) -> SYSTICK_SKEW_R {
        SYSTICK_SKEW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Indicates that no alternative reference clock source has been integrated."]
    #[inline(always)]
    pub fn systick_noref(&self) -> SYSTICK_NOREF_R {
        SYSTICK_NOREF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Provides an integer value to compute a 10ms (100Hz) delay. For example, apply the value 0x07A11F if no reference is implemented, and FCLK is 50MHz."]
    #[inline(always)]
    pub fn systick_tenms(&mut self) -> SYSTICK_TENMS_W {
        SYSTICK_TENMS_W { w: self }
    }
    #[doc = "Bit 24 - Systick clock is an exact multiple of 10ms"]
    #[inline(always)]
    pub fn systick_skew(&mut self) -> SYSTICK_SKEW_W {
        SYSTICK_SKEW_W { w: self }
    }
    #[doc = "Bit 25 - Indicates that no alternative reference clock source has been integrated."]
    #[inline(always)]
    pub fn systick_noref(&mut self) -> SYSTICK_NOREF_W {
        SYSTICK_NOREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systick_reg](index.html) module"]
pub struct SYSTICK_REG_SPEC;
impl crate::RegisterSpec for SYSTICK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systick_reg::R](R) reader structure"]
impl crate::Readable for SYSTICK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systick_reg::W](W) writer structure"]
impl crate::Writable for SYSTICK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTICK_REG to value 0x0200_0000"]
impl crate::Resettable for SYSTICK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
