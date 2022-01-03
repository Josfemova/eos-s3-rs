#[doc = "Register `OSC_CTRL_1` reader"]
pub struct R(crate::R<OSC_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL_1` writer"]
pub struct W(crate::W<OSC_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_1_SPEC>;
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
impl From<crate::W<OSC_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `prog` reader - Please refer to the Technical Reference Manual for others (No SYNC Needed). Power On Default Value is 76.97 MHz. No Support on 'Delta Mode'."]
pub struct PROG_R(crate::FieldReader<u16, u16>);
impl PROG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `prog` writer - Please refer to the Technical Reference Manual for others (No SYNC Needed). Power On Default Value is 76.97 MHz. No Support on 'Delta Mode'."]
pub struct PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
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
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Please refer to the Technical Reference Manual for others (No SYNC Needed). Power On Default Value is 76.97 MHz. No Support on 'Delta Mode'."]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - No description given"]
    #[inline(always)]
    pub fn general_purpos_sfr(&self) -> GENERAL_PURPOS_SFR_R {
        GENERAL_PURPOS_SFR_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Please refer to the Technical Reference Manual for others (No SYNC Needed). Power On Default Value is 76.97 MHz. No Support on 'Delta Mode'."]
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W {
        PROG_W { w: self }
    }
    #[doc = "Bits 13:15 - No description given"]
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
#[doc = "Oscilator control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl_1](index.html) module"]
pub struct OSC_CTRL_1_SPEC;
impl crate::RegisterSpec for OSC_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl_1::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl_1::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL_1 to value 0x092d"]
impl crate::Resettable for OSC_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x092d
    }
}
