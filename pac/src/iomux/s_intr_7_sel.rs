#[doc = "Register `S_INTR_7_SEL` reader"]
pub struct R(crate::R<S_INTR_7_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_INTR_7_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_INTR_7_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_INTR_7_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_INTR_7_SEL` writer"]
pub struct W(crate::W<S_INTR_7_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_INTR_7_SEL_SPEC>;
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
impl From<crate::W<S_INTR_7_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_INTR_7_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: undefined (0:0 in documentation)"]
    UNDEFINED = 0,
    #[doc = "1: Selects pad #13 as the trigger"]
    PAD_13 = 1,
    #[doc = "2: Selects pad #23 as the trigger"]
    PAD_23 = 2,
    #[doc = "3: Selects pad #33 as the trigger"]
    PAD_33 = 3,
    #[doc = "4: Selects pad #34 as the trigger"]
    PAD_34 = 4,
    #[doc = "5: Selects pad #42 as the trigger"]
    PAD_42 = 5,
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
            1 => Some(SEL_A::PAD_13),
            2 => Some(SEL_A::PAD_23),
            3 => Some(SEL_A::PAD_33),
            4 => Some(SEL_A::PAD_34),
            5 => Some(SEL_A::PAD_42),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        **self == SEL_A::UNDEFINED
    }
    #[doc = "Checks if the value of the field is `PAD_13`"]
    #[inline(always)]
    pub fn is_pad_13(&self) -> bool {
        **self == SEL_A::PAD_13
    }
    #[doc = "Checks if the value of the field is `PAD_23`"]
    #[inline(always)]
    pub fn is_pad_23(&self) -> bool {
        **self == SEL_A::PAD_23
    }
    #[doc = "Checks if the value of the field is `PAD_33`"]
    #[inline(always)]
    pub fn is_pad_33(&self) -> bool {
        **self == SEL_A::PAD_33
    }
    #[doc = "Checks if the value of the field is `PAD_34`"]
    #[inline(always)]
    pub fn is_pad_34(&self) -> bool {
        **self == SEL_A::PAD_34
    }
    #[doc = "Checks if the value of the field is `PAD_42`"]
    #[inline(always)]
    pub fn is_pad_42(&self) -> bool {
        **self == SEL_A::PAD_42
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
    #[doc = "Selects pad #13 as the trigger"]
    #[inline(always)]
    pub fn pad_13(self) -> &'a mut W {
        self.variant(SEL_A::PAD_13)
    }
    #[doc = "Selects pad #23 as the trigger"]
    #[inline(always)]
    pub fn pad_23(self) -> &'a mut W {
        self.variant(SEL_A::PAD_23)
    }
    #[doc = "Selects pad #33 as the trigger"]
    #[inline(always)]
    pub fn pad_33(self) -> &'a mut W {
        self.variant(SEL_A::PAD_33)
    }
    #[doc = "Selects pad #34 as the trigger"]
    #[inline(always)]
    pub fn pad_34(self) -> &'a mut W {
        self.variant(SEL_A::PAD_34)
    }
    #[doc = "Selects pad #42 as the trigger"]
    #[inline(always)]
    pub fn pad_42(self) -> &'a mut W {
        self.variant(SEL_A::PAD_42)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sel"]
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
#[doc = "Select pad that triggers GPIO interrupt 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_intr_7_sel](index.html) module"]
pub struct S_INTR_7_SEL_SPEC;
impl crate::RegisterSpec for S_INTR_7_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_intr_7_sel::R](R) reader structure"]
impl crate::Readable for S_INTR_7_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_intr_7_sel::W](W) writer structure"]
impl crate::Writable for S_INTR_7_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_INTR_7_SEL to value 0"]
impl crate::Resettable for S_INTR_7_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
