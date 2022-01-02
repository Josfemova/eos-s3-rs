#[doc = "Register `SW_CLK_SEL` reader"]
pub struct R(crate::R<SW_CLK_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CLK_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CLK_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CLK_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CLK_SEL` writer"]
pub struct W(crate::W<SW_CLK_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CLK_SEL_SPEC>;
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
impl From<crate::W<SW_CLK_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CLK_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "0: pad #14/#45 (pad selection will depend on pad #8, See ch. 30 of TechRef Manual)"]
    SELECT_STRAP_PAD_14_45 = 0,
    #[doc = "1: Lacking documentation. Might mean the same as `select_strap_pad`"]
    UNDEFINED = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Sel"]
pub struct SEL_R(crate::FieldReader<bool, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::SELECT_STRAP_PAD_14_45,
            true => SEL_A::UNDEFINED,
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_STRAP_PAD_14_45`"]
    #[inline(always)]
    pub fn is_select_strap_pad_14_45(&self) -> bool {
        **self == SEL_A::SELECT_STRAP_PAD_14_45
    }
    #[doc = "Checks if the value of the field is `UNDEFINED`"]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        **self == SEL_A::UNDEFINED
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, SEL_A>;
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
        self.bit(variant.into())
    }
    #[doc = "pad #14/#45 (pad selection will depend on pad #8, See ch. 30 of TechRef Manual)"]
    #[inline(always)]
    pub fn select_strap_pad_14_45(self) -> &'a mut W {
        self.variant(SEL_A::SELECT_STRAP_PAD_14_45)
    }
    #[doc = "Lacking documentation. Might mean the same as `select_strap_pad`"]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut W {
        self.variant(SEL_A::UNDEFINED)
    }
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
    #[doc = "Bit 0 - Sel"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sel"]
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
#[doc = "Selection for SWD clock pad (SCK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_clk_sel](index.html) module"]
pub struct SW_CLK_SEL_SPEC;
impl crate::RegisterSpec for SW_CLK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_clk_sel::R](R) reader structure"]
impl crate::Readable for SW_CLK_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_clk_sel::W](W) writer structure"]
impl crate::Writable for SW_CLK_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CLK_SEL to value 0"]
impl crate::Resettable for SW_CLK_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
