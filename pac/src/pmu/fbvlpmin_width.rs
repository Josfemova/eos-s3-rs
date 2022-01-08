#[doc = "Register `FBVLPMinWidth` reader"]
pub struct R(crate::R<FBVLPMINWIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBVLPMINWIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBVLPMINWIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBVLPMINWIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBVLPMinWidth` writer"]
pub struct W(crate::W<FBVLPMINWIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBVLPMINWIDTH_SPEC>;
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
impl From<crate::W<FBVLPMINWIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBVLPMINWIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBVLPMinWidth` reader - For FB, it required additional IDLE Cylcle from turning off power gates (Power is ON) to ready for normal operation. This is used to defined the # of the IDLE cycles (base on C01 Clock). According to FB spec, it will require 10uS. But for final spec, it should be from FB design team."]
pub struct FBVLPMINWIDTH_R(crate::FieldReader<u16, u16>);
impl FBVLPMINWIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FBVLPMINWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBVLPMINWIDTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBVLPMinWidth` writer - For FB, it required additional IDLE Cylcle from turning off power gates (Power is ON) to ready for normal operation. This is used to defined the # of the IDLE cycles (base on C01 Clock). According to FB spec, it will require 10uS. But for final spec, it should be from FB design team."]
pub struct FBVLPMINWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FBVLPMINWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - For FB, it required additional IDLE Cylcle from turning off power gates (Power is ON) to ready for normal operation. This is used to defined the # of the IDLE cycles (base on C01 Clock). According to FB spec, it will require 10uS. But for final spec, it should be from FB design team."]
    #[inline(always)]
    pub fn fbvlpmin_width(&self) -> FBVLPMINWIDTH_R {
        FBVLPMINWIDTH_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - For FB, it required additional IDLE Cylcle from turning off power gates (Power is ON) to ready for normal operation. This is used to defined the # of the IDLE cycles (base on C01 Clock). According to FB spec, it will require 10uS. But for final spec, it should be from FB design team."]
    #[inline(always)]
    pub fn fbvlpmin_width(&mut self) -> FBVLPMINWIDTH_W {
        FBVLPMINWIDTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for the amount of IDLE cycles before powering on the FB domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbvlpmin_width](index.html) module"]
pub struct FBVLPMINWIDTH_SPEC;
impl crate::RegisterSpec for FBVLPMINWIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbvlpmin_width::R](R) reader structure"]
impl crate::Readable for FBVLPMINWIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbvlpmin_width::W](W) writer structure"]
impl crate::Writable for FBVLPMINWIDTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBVLPMinWidth to value 0"]
impl crate::Resettable for FBVLPMINWIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
