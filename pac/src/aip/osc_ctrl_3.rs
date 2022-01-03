#[doc = "Register `OSC_CTRL_3` reader"]
pub struct R(crate::R<OSC_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_3` writer"]
pub struct W(crate::W<OSC_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_3_SPEC>;
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
impl From<crate::W<OSC_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `refok` reader - If 1'b1, will force the refok pin to 1, otherwise, it is control by the RTC/oscok"]
pub struct REFOK_R(crate::FieldReader<bool, bool>);
impl REFOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `refok` writer - If 1'b1, will force the refok pin to 1, otherwise, it is control by the RTC/oscok"]
pub struct REFOK_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOK_W<'a> {
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
#[doc = "Field `enmon` reader - Turn on Monitor function by default"]
pub struct ENMON_R(crate::FieldReader<bool, bool>);
impl ENMON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enmon` writer - Turn on Monitor function by default"]
pub struct ENMON_W<'a> {
    w: &'a mut W,
}
impl<'a> ENMON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `General_Purpos_SFR` reader - No description given"]
pub struct GENERAL_PURPOS_SFR_R(crate::FieldReader<u8, u8>);
impl GENERAL_PURPOS_SFR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GENERAL_PURPOS_SFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_PURPOS_SFR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `General_Purpos_SFR` writer - No description given"]
pub struct GENERAL_PURPOS_SFR_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERAL_PURPOS_SFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If 1'b1, will force the refok pin to 1, otherwise, it is control by the RTC/oscok"]
    #[inline(always)]
    pub fn refok(&self) -> REFOK_R {
        REFOK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Turn on Monitor function by default"]
    #[inline(always)]
    pub fn enmon(&self) -> ENMON_R {
        ENMON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - No description given"]
    #[inline(always)]
    pub fn general_purpos_sfr(&self) -> GENERAL_PURPOS_SFR_R {
        GENERAL_PURPOS_SFR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If 1'b1, will force the refok pin to 1, otherwise, it is control by the RTC/oscok"]
    #[inline(always)]
    pub fn refok(&mut self) -> REFOK_W {
        REFOK_W { w: self }
    }
    #[doc = "Bit 3 - Turn on Monitor function by default"]
    #[inline(always)]
    pub fn enmon(&mut self) -> ENMON_W {
        ENMON_W { w: self }
    }
    #[doc = "Bits 4:7 - No description given"]
    #[inline(always)]
    pub fn general_purpos_sfr(&mut self) -> GENERAL_PURPOS_SFR_W {
        GENERAL_PURPOS_SFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscilator control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_3](index.html) module"]
pub struct OSC_CTRL_3_SPEC;
impl crate::RegisterSpec for OSC_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_3::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_3::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_3 to value 0x08"]
impl crate::Resettable for OSC_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
