#[doc = "Register `IrDA_Sirin_SEL` reader"]
pub struct R(crate::R<IRDA_SIRIN_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRDA_SIRIN_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRDA_SIRIN_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRDA_SIRIN_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IrDA_Sirin_SEL` writer"]
pub struct W(crate::W<IRDA_SIRIN_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRDA_SIRIN_SEL_SPEC>;
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
impl From<crate::W<IRDA_SIRIN_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRDA_SIRIN_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: undefined (0:0 in documentation)"]
    UNDEFINED = 0,
    #[doc = "1: Selects pad #14 for Siren in line"]
    PAD_06 = 1,
    #[doc = "2: Selects pad #15 for Siren in line"]
    PAD_15 = 2,
    #[doc = "3: Selects pad #21 for Siren in line"]
    PAD_21 = 3,
    #[doc = "4: Selects pad #24 for Siren in line"]
    PAD_24 = 4,
    #[doc = "5: Selects pad #28 for Siren in line"]
    PAD_28 = 5,
    #[doc = "6: Selects pad #40 for Siren in line"]
    PAD_40 = 6,
    #[doc = "7: Selects pad #44 for Siren in line"]
    PAD_44 = 7,
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
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::UNDEFINED,
            1 => SEL_A::PAD_06,
            2 => SEL_A::PAD_15,
            3 => SEL_A::PAD_21,
            4 => SEL_A::PAD_24,
            5 => SEL_A::PAD_28,
            6 => SEL_A::PAD_40,
            7 => SEL_A::PAD_44,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        **self == SEL_A::UNDEFINED
    }
    #[doc = "Checks if the value of the field is `PAD_06`"]
    #[inline(always)]
    pub fn is_pad_06(&self) -> bool {
        **self == SEL_A::PAD_06
    }
    #[doc = "Checks if the value of the field is `PAD_15`"]
    #[inline(always)]
    pub fn is_pad_15(&self) -> bool {
        **self == SEL_A::PAD_15
    }
    #[doc = "Checks if the value of the field is `PAD_21`"]
    #[inline(always)]
    pub fn is_pad_21(&self) -> bool {
        **self == SEL_A::PAD_21
    }
    #[doc = "Checks if the value of the field is `PAD_24`"]
    #[inline(always)]
    pub fn is_pad_24(&self) -> bool {
        **self == SEL_A::PAD_24
    }
    #[doc = "Checks if the value of the field is `PAD_28`"]
    #[inline(always)]
    pub fn is_pad_28(&self) -> bool {
        **self == SEL_A::PAD_28
    }
    #[doc = "Checks if the value of the field is `PAD_40`"]
    #[inline(always)]
    pub fn is_pad_40(&self) -> bool {
        **self == SEL_A::PAD_40
    }
    #[doc = "Checks if the value of the field is `PAD_44`"]
    #[inline(always)]
    pub fn is_pad_44(&self) -> bool {
        **self == SEL_A::PAD_44
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
        self.bits(variant.into())
    }
    #[doc = "undefined (0:0 in documentation)"]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut W {
        self.variant(SEL_A::UNDEFINED)
    }
    #[doc = "Selects pad #14 for Siren in line"]
    #[inline(always)]
    pub fn pad_06(self) -> &'a mut W {
        self.variant(SEL_A::PAD_06)
    }
    #[doc = "Selects pad #15 for Siren in line"]
    #[inline(always)]
    pub fn pad_15(self) -> &'a mut W {
        self.variant(SEL_A::PAD_15)
    }
    #[doc = "Selects pad #21 for Siren in line"]
    #[inline(always)]
    pub fn pad_21(self) -> &'a mut W {
        self.variant(SEL_A::PAD_21)
    }
    #[doc = "Selects pad #24 for Siren in line"]
    #[inline(always)]
    pub fn pad_24(self) -> &'a mut W {
        self.variant(SEL_A::PAD_24)
    }
    #[doc = "Selects pad #28 for Siren in line"]
    #[inline(always)]
    pub fn pad_28(self) -> &'a mut W {
        self.variant(SEL_A::PAD_28)
    }
    #[doc = "Selects pad #40 for Siren in line"]
    #[inline(always)]
    pub fn pad_40(self) -> &'a mut W {
        self.variant(SEL_A::PAD_40)
    }
    #[doc = "Selects pad #44 for Siren in line"]
    #[inline(always)]
    pub fn pad_44(self) -> &'a mut W {
        self.variant(SEL_A::PAD_44)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
#[doc = "Select pad for SIREN in function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir_da_sirin_sel](index.html) module"]
pub struct IRDA_SIRIN_SEL_SPEC;
impl crate::RegisterSpec for IRDA_SIRIN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir_da_sirin_sel::R](R) reader structure"]
impl crate::Readable for IRDA_SIRIN_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir_da_sirin_sel::W](W) writer structure"]
impl crate::Writable for IRDA_SIRIN_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IrDA_Sirin_SEL to value 0"]
impl crate::Resettable for IRDA_SIRIN_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
