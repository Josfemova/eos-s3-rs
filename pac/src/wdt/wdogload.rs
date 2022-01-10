#[doc = "Register `WDOGLOAD` reader"]
pub struct R(crate::R<WDOGLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGLOAD` writer"]
pub struct W(crate::W<WDOGLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGLOAD_SPEC>;
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
impl From<crate::W<WDOGLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDGLOAD` reader - Load value of the WhatchDog timer"]
pub struct WDGLOAD_R(crate::FieldReader<u32, u32>);
impl WDGLOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WDGLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDGLOAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDGLOAD` writer - Load value of the WhatchDog timer"]
pub struct WDGLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Load value of the WhatchDog timer"]
    #[inline(always)]
    pub fn wdgload(&self) -> WDGLOAD_R {
        WDGLOAD_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Load value of the WhatchDog timer"]
    #[inline(always)]
    pub fn wdgload(&mut self) -> WDGLOAD_W {
        WDGLOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The WDOGLOAD Register contains the value from which the counter is to decrement. When this register is written to, the count is immediately restarted from the new value. The minimum valid value for WDOGLOAD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogload](index.html) module"]
pub struct WDOGLOAD_SPEC;
impl crate::RegisterSpec for WDOGLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogload::R](R) reader structure"]
impl crate::Readable for WDOGLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogload::W](W) writer structure"]
impl crate::Writable for WDOGLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGLOAD to value 0xffff_ffff"]
impl crate::Resettable for WDOGLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
