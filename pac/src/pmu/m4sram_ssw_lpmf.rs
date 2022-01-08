#[doc = "Register `M4SRAM_SSW_LPMF` reader"]
pub struct R(crate::R<M4SRAM_SSW_LPMF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4SRAM_SSW_LPMF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4SRAM_SSW_LPMF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4SRAM_SSW_LPMF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4SRAM_SSW_LPMF` writer"]
pub struct W(crate::W<M4SRAM_SSW_LPMF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4SRAM_SSW_LPMF_SPEC>;
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
impl From<crate::W<M4SRAM_SSW_LPMF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4SRAM_SSW_LPMF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable Light Sleep Mode. If M4 SRAM power is removing, the corresponding bit will be clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum M4SRAM_LPMF_A {
    #[doc = "1: Enable M4 SRAM Light Sleep Mode."]
    ENABLE_LIGHT_SLEEP = 1,
}
impl From<M4SRAM_LPMF_A> for u16 {
    #[inline(always)]
    fn from(variant: M4SRAM_LPMF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `M4SRAM_LPMF` reader - Enable or disable Light Sleep Mode. If M4 SRAM power is removing, the corresponding bit will be clear."]
pub struct M4SRAM_LPMF_R(crate::FieldReader<u16, M4SRAM_LPMF_A>);
impl M4SRAM_LPMF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        M4SRAM_LPMF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M4SRAM_LPMF_A> {
        match self.bits {
            1 => Some(M4SRAM_LPMF_A::ENABLE_LIGHT_SLEEP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_LIGHT_SLEEP`"]
    #[inline(always)]
    pub fn is_enable_light_sleep(&self) -> bool {
        **self == M4SRAM_LPMF_A::ENABLE_LIGHT_SLEEP
    }
}
impl core::ops::Deref for M4SRAM_LPMF_R {
    type Target = crate::FieldReader<u16, M4SRAM_LPMF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4SRAM_LPMF` writer - Enable or disable Light Sleep Mode. If M4 SRAM power is removing, the corresponding bit will be clear."]
pub struct M4SRAM_LPMF_W<'a> {
    w: &'a mut W,
}
impl<'a> M4SRAM_LPMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4SRAM_LPMF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable M4 SRAM Light Sleep Mode."]
    #[inline(always)]
    pub fn enable_light_sleep(self) -> &'a mut W {
        self.variant(M4SRAM_LPMF_A::ENABLE_LIGHT_SLEEP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Enable or disable Light Sleep Mode. If M4 SRAM power is removing, the corresponding bit will be clear."]
    #[inline(always)]
    pub fn m4sram_lpmf(&self) -> M4SRAM_LPMF_R {
        M4SRAM_LPMF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enable or disable Light Sleep Mode. If M4 SRAM power is removing, the corresponding bit will be clear."]
    #[inline(always)]
    pub fn m4sram_lpmf(&mut self) -> M4SRAM_LPMF_W {
        M4SRAM_LPMF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control for M4SRAM power domain light sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4sram_ssw_lpmf](index.html) module"]
pub struct M4SRAM_SSW_LPMF_SPEC;
impl crate::RegisterSpec for M4SRAM_SSW_LPMF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4sram_ssw_lpmf::R](R) reader structure"]
impl crate::Readable for M4SRAM_SSW_LPMF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4sram_ssw_lpmf::W](W) writer structure"]
impl crate::Writable for M4SRAM_SSW_LPMF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4SRAM_SSW_LPMF to value 0"]
impl crate::Resettable for M4SRAM_SSW_LPMF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
