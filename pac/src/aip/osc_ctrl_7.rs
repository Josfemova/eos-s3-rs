#[doc = "Register `OSC_CTRL_7` reader"]
pub struct R(crate::R<OSC_CTRL_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_7` writer"]
pub struct W(crate::W<OSC_CTRL_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_7_SPEC>;
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
impl From<crate::W<OSC_CTRL_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_7_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscilator control register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_7](index.html) module"]
pub struct OSC_CTRL_7_SPEC;
impl crate::RegisterSpec for OSC_CTRL_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_7::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_7::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_7 to value 0"]
impl crate::Resettable for OSC_CTRL_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
