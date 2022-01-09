#[doc = "Register `FFE_DEBUG_SEL` reader"]
pub struct R(crate::R<FFE_DEBUG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_DEBUG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_DEBUG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_DEBUG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_DEBUG_SEL` writer"]
pub struct W(crate::W<FFE_DEBUG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_DEBUG_SEL_SPEC>;
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
impl From<crate::W<FFE_DEBUG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_DEBUG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFE_DEBUG_SEL_SM0` reader - SM0 LS debug slection"]
pub struct FFE_DEBUG_SEL_SM0_R(crate::FieldReader<u8, u8>);
impl FFE_DEBUG_SEL_SM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFE_DEBUG_SEL_SM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_DEBUG_SEL_SM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_DEBUG_SEL_SM0` writer - SM0 LS debug slection"]
pub struct FFE_DEBUG_SEL_SM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_DEBUG_SEL_SM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `FFE_DEBUG_SEL_SM1` reader - SM1 LS debug slection"]
pub struct FFE_DEBUG_SEL_SM1_R(crate::FieldReader<u8, u8>);
impl FFE_DEBUG_SEL_SM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFE_DEBUG_SEL_SM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_DEBUG_SEL_SM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_DEBUG_SEL_SM1` writer - SM1 LS debug slection"]
pub struct FFE_DEBUG_SEL_SM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_DEBUG_SEL_SM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FFE_DEBUG_SEL_FFE0` reader - FFE0 LS debug slection"]
pub struct FFE_DEBUG_SEL_FFE0_R(crate::FieldReader<u8, u8>);
impl FFE_DEBUG_SEL_FFE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFE_DEBUG_SEL_FFE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE_DEBUG_SEL_FFE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_DEBUG_SEL_FFE0` writer - FFE0 LS debug slection"]
pub struct FFE_DEBUG_SEL_FFE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_DEBUG_SEL_FFE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "FFE_TOP_MS debug slection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FFE_TOP_DEBUG_SEL_A {
    #[doc = "0: Select sm0"]
    SM0 = 0,
    #[doc = "1: Select sm1"]
    SM1 = 1,
    #[doc = "2: Select ffe0"]
    FFE0 = 2,
}
impl From<FFE_TOP_DEBUG_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FFE_TOP_DEBUG_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FFE_TOP_DEBUG_SEL` reader - FFE_TOP_MS debug slection"]
pub struct FFE_TOP_DEBUG_SEL_R(crate::FieldReader<u8, FFE_TOP_DEBUG_SEL_A>);
impl FFE_TOP_DEBUG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFE_TOP_DEBUG_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FFE_TOP_DEBUG_SEL_A> {
        match self.bits {
            0 => Some(FFE_TOP_DEBUG_SEL_A::SM0),
            1 => Some(FFE_TOP_DEBUG_SEL_A::SM1),
            2 => Some(FFE_TOP_DEBUG_SEL_A::FFE0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SM0`"]
    #[inline(always)]
    pub fn is_sm0(&self) -> bool {
        **self == FFE_TOP_DEBUG_SEL_A::SM0
    }
    #[doc = "Checks if the value of the field is `SM1`"]
    #[inline(always)]
    pub fn is_sm1(&self) -> bool {
        **self == FFE_TOP_DEBUG_SEL_A::SM1
    }
    #[doc = "Checks if the value of the field is `FFE0`"]
    #[inline(always)]
    pub fn is_ffe0(&self) -> bool {
        **self == FFE_TOP_DEBUG_SEL_A::FFE0
    }
}
impl core::ops::Deref for FFE_TOP_DEBUG_SEL_R {
    type Target = crate::FieldReader<u8, FFE_TOP_DEBUG_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE_TOP_DEBUG_SEL` writer - FFE_TOP_MS debug slection"]
pub struct FFE_TOP_DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE_TOP_DEBUG_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE_TOP_DEBUG_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select sm0"]
    #[inline(always)]
    pub fn sm0(self) -> &'a mut W {
        self.variant(FFE_TOP_DEBUG_SEL_A::SM0)
    }
    #[doc = "Select sm1"]
    #[inline(always)]
    pub fn sm1(self) -> &'a mut W {
        self.variant(FFE_TOP_DEBUG_SEL_A::SM1)
    }
    #[doc = "Select ffe0"]
    #[inline(always)]
    pub fn ffe0(self) -> &'a mut W {
        self.variant(FFE_TOP_DEBUG_SEL_A::FFE0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SM0 LS debug slection"]
    #[inline(always)]
    pub fn ffe_debug_sel_sm0(&self) -> FFE_DEBUG_SEL_SM0_R {
        FFE_DEBUG_SEL_SM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SM1 LS debug slection"]
    #[inline(always)]
    pub fn ffe_debug_sel_sm1(&self) -> FFE_DEBUG_SEL_SM1_R {
        FFE_DEBUG_SEL_SM1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FFE0 LS debug slection"]
    #[inline(always)]
    pub fn ffe_debug_sel_ffe0(&self) -> FFE_DEBUG_SEL_FFE0_R {
        FFE_DEBUG_SEL_FFE0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - FFE_TOP_MS debug slection"]
    #[inline(always)]
    pub fn ffe_top_debug_sel(&self) -> FFE_TOP_DEBUG_SEL_R {
        FFE_TOP_DEBUG_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SM0 LS debug slection"]
    #[inline(always)]
    pub fn ffe_debug_sel_sm0(&mut self) -> FFE_DEBUG_SEL_SM0_W {
        FFE_DEBUG_SEL_SM0_W { w: self }
    }
    #[doc = "Bits 8:15 - SM1 LS debug slection"]
    #[inline(always)]
    pub fn ffe_debug_sel_sm1(&mut self) -> FFE_DEBUG_SEL_SM1_W {
        FFE_DEBUG_SEL_SM1_W { w: self }
    }
    #[doc = "Bits 16:23 - FFE0 LS debug slection"]
    #[inline(always)]
    pub fn ffe_debug_sel_ffe0(&mut self) -> FFE_DEBUG_SEL_FFE0_W {
        FFE_DEBUG_SEL_FFE0_W { w: self }
    }
    #[doc = "Bits 24:25 - FFE_TOP_MS debug slection"]
    #[inline(always)]
    pub fn ffe_top_debug_sel(&mut self) -> FFE_TOP_DEBUG_SEL_W {
        FFE_TOP_DEBUG_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_debug_sel](index.html) module"]
pub struct FFE_DEBUG_SEL_SPEC;
impl crate::RegisterSpec for FFE_DEBUG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_debug_sel::R](R) reader structure"]
impl crate::Readable for FFE_DEBUG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_debug_sel::W](W) writer structure"]
impl crate::Writable for FFE_DEBUG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_DEBUG_SEL to value 0"]
impl crate::Resettable for FFE_DEBUG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
