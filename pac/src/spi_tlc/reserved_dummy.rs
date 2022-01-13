#[doc = "Register `Reserved_dummy` writer"]
pub struct W(crate::W<RESERVED_DUMMY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESERVED_DUMMY_SPEC>;
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
impl From<crate::W<RESERVED_DUMMY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESERVED_DUMMY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dummy` writer - For dummy write purpose"]
pub struct DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - For dummy write purpose"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DUMMY_W {
        DUMMY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For dummy write purpose.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved_dummy](index.html) module"]
pub struct RESERVED_DUMMY_SPEC;
impl crate::RegisterSpec for RESERVED_DUMMY_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [reserved_dummy::W](W) writer structure"]
impl crate::Writable for RESERVED_DUMMY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Reserved_dummy to value 0"]
impl crate::Resettable for RESERVED_DUMMY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
