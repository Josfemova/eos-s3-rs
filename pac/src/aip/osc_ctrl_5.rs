#[doc = "Register `OSC_CTRL_5` reader"]
pub struct R(crate::R<OSC_CTRL_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_5` writer"]
pub struct W(crate::W<OSC_CTRL_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_5_SPEC>;
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
impl From<crate::W<OSC_CTRL_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdi` reader - Please refer to the Technical Reference Manual for detail"]
pub struct SDI_R(crate::FieldReader<bool, bool>);
impl SDI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdi` writer - Please refer to the Technical Reference Manual for detail"]
pub struct SDI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDI_W<'a> {
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
    #[doc = "Bit 0 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn sdi(&self) -> SDI_R {
        SDI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn sdi(&mut self) -> SDI_W {
        SDI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscilator control register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_5](index.html) module"]
pub struct OSC_CTRL_5_SPEC;
impl crate::RegisterSpec for OSC_CTRL_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_5::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_5::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_5 to value 0"]
impl crate::Resettable for OSC_CTRL_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
