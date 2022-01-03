#[doc = "Register `OSC_CTRL_6` reader"]
pub struct R(crate::R<OSC_CTRL_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_6` writer"]
pub struct W(crate::W<OSC_CTRL_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_6_SPEC>;
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
impl From<crate::W<OSC_CTRL_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sck` reader - Please refer to the Technical Reference Manual for detail"]
pub struct SCK_R(crate::FieldReader<bool, bool>);
impl SCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sck` writer - Please refer to the Technical Reference Manual for detail"]
pub struct SCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_W<'a> {
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
    pub fn sck(&self) -> SCK_R {
        SCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn sck(&mut self) -> SCK_W {
        SCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscilator control register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_6](index.html) module"]
pub struct OSC_CTRL_6_SPEC;
impl crate::RegisterSpec for OSC_CTRL_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_6::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_6::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_6 to value 0"]
impl crate::Resettable for OSC_CTRL_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
