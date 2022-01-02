#[doc = "Register `PDM_DATA_SELE` reader"]
pub struct R(crate::R<PDM_DATA_SELE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_DATA_SELE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_DATA_SELE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_DATA_SELE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_DATA_SELE` writer"]
pub struct W(crate::W<PDM_DATA_SELE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_DATA_SELE_SPEC>;
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
impl From<crate::W<PDM_DATA_SELE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_DATA_SELE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: undefined (0:0 in documentation)"]
    UNDEFINED = 0,
    #[doc = "1: Selects pad #10 for PDM data line"]
    PAD_10 = 1,
    #[doc = "2: Selects pad #28 for PDM data line"]
    PAD_28 = 2,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::UNDEFINED),
            1 => Some(SEL_A::PAD_10),
            2 => Some(SEL_A::PAD_28),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        **self == SEL_A::UNDEFINED
    }
    #[doc = "Checks if the value of the field is `PAD_10`"]
    #[inline(always)]
    pub fn is_pad_10(&self) -> bool {
        **self == SEL_A::PAD_10
    }
    #[doc = "Checks if the value of the field is `PAD_28`"]
    #[inline(always)]
    pub fn is_pad_28(&self) -> bool {
        **self == SEL_A::PAD_28
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Sel"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "undefined (0:0 in documentation)"]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut W {
        self.variant(SEL_A::UNDEFINED)
    }
    #[doc = "Selects pad #10 for PDM data line"]
    #[inline(always)]
    pub fn pad_10(self) -> &'a mut W {
        self.variant(SEL_A::PAD_10)
    }
    #[doc = "Selects pad #28 for PDM data line"]
    #[inline(always)]
    pub fn pad_28(self) -> &'a mut W {
        self.variant(SEL_A::PAD_28)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sel"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select pad for PDM microphone data line\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_data_sele](index.html) module"]
pub struct PDM_DATA_SELE_SPEC;
impl crate::RegisterSpec for PDM_DATA_SELE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_data_sele::R](R) reader structure"]
impl crate::Readable for PDM_DATA_SELE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_data_sele::W](W) writer structure"]
impl crate::Writable for PDM_DATA_SELE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_DATA_SELE to value 0"]
impl crate::Resettable for PDM_DATA_SELE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
