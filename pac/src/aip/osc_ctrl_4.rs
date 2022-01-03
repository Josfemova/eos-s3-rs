#[doc = "Register `OSC_CTRL_4` reader"]
pub struct R(crate::R<OSC_CTRL_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_4` writer"]
pub struct W(crate::W<OSC_CTRL_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_4_SPEC>;
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
impl From<crate::W<OSC_CTRL_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `test` reader - Please refer to the Technical Reference Manual for detail"]
pub struct TEST_R(crate::FieldReader<u8, u8>);
impl TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test` writer - Please refer to the Technical Reference Manual for detail"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `ce` reader - Please refer to the Technical Reference Manual for detail"]
pub struct CE_R(crate::FieldReader<u8, u8>);
impl CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ce` writer - Please refer to the Technical Reference Manual for detail"]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `wr` reader - Please refer to the Technical Reference Manual for detail"]
pub struct WR_R(crate::FieldReader<bool, bool>);
impl WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wr` writer - Please refer to the Technical Reference Manual for detail"]
pub struct WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bits 3:4 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
    #[doc = "Bit 5 - Please refer to the Technical Reference Manual for detail"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W {
        WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscilator control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_4](index.html) module"]
pub struct OSC_CTRL_4_SPEC;
impl crate::RegisterSpec for OSC_CTRL_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_4::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_4::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_4 to value 0"]
impl crate::Resettable for OSC_CTRL_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
