#[doc = "Register `FBIO_SEL_2` reader"]
pub struct R(crate::R<FBIO_SEL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBIO_SEL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBIO_SEL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBIO_SEL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBIO_SEL_2` writer"]
pub struct W(crate::W<FBIO_SEL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBIO_SEL_2_SPEC>;
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
impl From<crate::W<FBIO_SEL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBIO_SEL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SEL_A {
    #[doc = "0: Lacking documentation. Guess: Leaves pad unselected"]
    UNDEFINED = 0,
    #[doc = "1: Lacking documentation. Guess: Activates some sort of selection for pads 32-45"]
    SELECT_PAD = 1,
}
impl From<SEL_A> for u16 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<u16, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::UNDEFINED),
            1 => Some(SEL_A::SELECT_PAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        **self == SEL_A::UNDEFINED
    }
    #[doc = "Checks if the value of the field is `SELECT_PAD`"]
    #[inline(always)]
    pub fn is_select_pad(&self) -> bool {
        **self == SEL_A::SELECT_PAD
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u16, SEL_A>;
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
    #[doc = "Lacking documentation. Guess: Leaves pad unselected"]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut W {
        self.variant(SEL_A::UNDEFINED)
    }
    #[doc = "Lacking documentation. Guess: Activates some sort of selection for pads 32-45"]
    #[inline(always)]
    pub fn select_pad(self) -> &'a mut W {
        self.variant(SEL_A::SELECT_PAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Sel"]
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
#[doc = "Lacking proper documentation. Configuration of pins 32-45 related to Fabric\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbio_sel_2](index.html) module"]
pub struct FBIO_SEL_2_SPEC;
impl crate::RegisterSpec for FBIO_SEL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbio_sel_2::R](R) reader structure"]
impl crate::Readable for FBIO_SEL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbio_sel_2::W](W) writer structure"]
impl crate::Writable for FBIO_SEL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBIO_SEL_2 to value 0"]
impl crate::Resettable for FBIO_SEL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
