#[doc = "Register `FB_ISOLATION` reader"]
pub struct R(crate::R<FB_ISOLATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_ISOLATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_ISOLATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_ISOLATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_ISOLATION` writer"]
pub struct W(crate::W<FB_ISOLATION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_ISOLATION_SPEC>;
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
impl From<crate::W<FB_ISOLATION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_ISOLATION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Enable_the_FB_Isolation` reader - This bit will be set to 1 if FB is waking up from shut down mode. The FB interface wil be kept in isolation the domain, except for the FB configuration interface itself"]
pub struct ENABLE_THE_FB_ISOLATION_R(crate::FieldReader<bool, bool>);
impl ENABLE_THE_FB_ISOLATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_THE_FB_ISOLATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_THE_FB_ISOLATION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Enable_the_FB_Isolation` writer - This bit will be set to 1 if FB is waking up from shut down mode. The FB interface wil be kept in isolation the domain, except for the FB configuration interface itself"]
pub struct ENABLE_THE_FB_ISOLATION_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_THE_FB_ISOLATION_W<'a> {
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
    #[doc = "Bit 0 - This bit will be set to 1 if FB is waking up from shut down mode. The FB interface wil be kept in isolation the domain, except for the FB configuration interface itself"]
    #[inline(always)]
    pub fn enable_the_fb_isolation(&self) -> ENABLE_THE_FB_ISOLATION_R {
        ENABLE_THE_FB_ISOLATION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit will be set to 1 if FB is waking up from shut down mode. The FB interface wil be kept in isolation the domain, except for the FB configuration interface itself"]
    #[inline(always)]
    pub fn enable_the_fb_isolation(&mut self) -> ENABLE_THE_FB_ISOLATION_W {
        ENABLE_THE_FB_ISOLATION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the FB Isolation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_isolation](index.html) module"]
pub struct FB_ISOLATION_SPEC;
impl crate::RegisterSpec for FB_ISOLATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_isolation::R](R) reader structure"]
impl crate::Readable for FB_ISOLATION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_isolation::W](W) writer structure"]
impl crate::Writable for FB_ISOLATION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_ISOLATION to value 0"]
impl crate::Resettable for FB_ISOLATION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
