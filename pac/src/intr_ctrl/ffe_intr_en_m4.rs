#[doc = "Register `FFE_INTR_EN_M4` reader"]
pub struct R(crate::R<FFE_INTR_EN_M4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_INTR_EN_M4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_INTR_EN_M4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_INTR_EN_M4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_INTR_EN_M4` writer"]
pub struct W(crate::W<FFE_INTR_EN_M4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_INTR_EN_M4_SPEC>;
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
impl From<crate::W<FFE_INTR_EN_M4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_INTR_EN_M4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FFE0_0 interrupt enable for M4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFE0_0_INTR_EN_M4_A {
    #[doc = "0: Disable the interrupt for the power domain"]
    ENABLE = 0,
    #[doc = "1: Enable the interrupt for the power domain"]
    DISABLE = 1,
}
impl From<FFE0_0_INTR_EN_M4_A> for bool {
    #[inline(always)]
    fn from(variant: FFE0_0_INTR_EN_M4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFE0_0_INTR_EN_M4` reader - FFE0_0 interrupt enable for M4"]
pub struct FFE0_0_INTR_EN_M4_R(crate::FieldReader<bool, FFE0_0_INTR_EN_M4_A>);
impl FFE0_0_INTR_EN_M4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_0_INTR_EN_M4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFE0_0_INTR_EN_M4_A {
        match self.bits {
            false => FFE0_0_INTR_EN_M4_A::ENABLE,
            true => FFE0_0_INTR_EN_M4_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FFE0_0_INTR_EN_M4_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FFE0_0_INTR_EN_M4_A::DISABLE
    }
}
impl core::ops::Deref for FFE0_0_INTR_EN_M4_R {
    type Target = crate::FieldReader<bool, FFE0_0_INTR_EN_M4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_0_INTR_EN_M4` writer - FFE0_0 interrupt enable for M4"]
pub struct FFE0_0_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_0_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_0_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_0_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_0_INTR_EN_M4_A::DISABLE)
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
#[doc = "FFE0_1 interrupt enable for M4"]
pub type FFE0_1_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_1_INTR_EN_M4` reader - FFE0_1 interrupt enable for M4"]
pub type FFE0_1_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_1_INTR_EN_M4` writer - FFE0_1 interrupt enable for M4"]
pub struct FFE0_1_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_1_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_1_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_1_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_1_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "FFE0_2 interrupt enable for M4"]
pub type FFE0_2_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_2_INTR_EN_M4` reader - FFE0_2 interrupt enable for M4"]
pub type FFE0_2_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_2_INTR_EN_M4` writer - FFE0_2 interrupt enable for M4"]
pub struct FFE0_2_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_2_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_2_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_2_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_2_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "FFE0_3 interrupt enable for M4"]
pub type FFE0_3_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_3_INTR_EN_M4` reader - FFE0_3 interrupt enable for M4"]
pub type FFE0_3_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_3_INTR_EN_M4` writer - FFE0_3 interrupt enable for M4"]
pub struct FFE0_3_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_3_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_3_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_3_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_3_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "FFE0_4 interrupt enable for M4"]
pub type FFE0_4_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_4_INTR_EN_M4` reader - FFE0_4 interrupt enable for M4"]
pub type FFE0_4_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_4_INTR_EN_M4` writer - FFE0_4 interrupt enable for M4"]
pub struct FFE0_4_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_4_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_4_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_4_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_4_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "FFE0_5 interrupt enable for M4"]
pub type FFE0_5_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_5_INTR_EN_M4` reader - FFE0_5 interrupt enable for M4"]
pub type FFE0_5_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_5_INTR_EN_M4` writer - FFE0_5 interrupt enable for M4"]
pub struct FFE0_5_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_5_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_5_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_5_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_5_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "FFE0_6 interrupt enable for M4"]
pub type FFE0_6_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_6_INTR_EN_M4` reader - FFE0_6 interrupt enable for M4"]
pub type FFE0_6_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_6_INTR_EN_M4` writer - FFE0_6 interrupt enable for M4"]
pub struct FFE0_6_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_6_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_6_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_6_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_6_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "FFE0_7 interrupt enable for M4"]
pub type FFE0_7_INTR_EN_M4_A = FFE0_0_INTR_EN_M4_A;
#[doc = "Field `FFE0_7_INTR_EN_M4` reader - FFE0_7 interrupt enable for M4"]
pub type FFE0_7_INTR_EN_M4_R = FFE0_0_INTR_EN_M4_R;
#[doc = "Field `FFE0_7_INTR_EN_M4` writer - FFE0_7 interrupt enable for M4"]
pub struct FFE0_7_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_7_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_7_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_7_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_7_INTR_EN_M4_A::DISABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FFE0_0 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_0_intr_en_m4(&self) -> FFE0_0_INTR_EN_M4_R {
        FFE0_0_INTR_EN_M4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FFE0_1 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_1_intr_en_m4(&self) -> FFE0_1_INTR_EN_M4_R {
        FFE0_1_INTR_EN_M4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FFE0_2 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_2_intr_en_m4(&self) -> FFE0_2_INTR_EN_M4_R {
        FFE0_2_INTR_EN_M4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FFE0_3 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_3_intr_en_m4(&self) -> FFE0_3_INTR_EN_M4_R {
        FFE0_3_INTR_EN_M4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FFE0_4 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_4_intr_en_m4(&self) -> FFE0_4_INTR_EN_M4_R {
        FFE0_4_INTR_EN_M4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FFE0_5 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_5_intr_en_m4(&self) -> FFE0_5_INTR_EN_M4_R {
        FFE0_5_INTR_EN_M4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FFE0_6 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_6_intr_en_m4(&self) -> FFE0_6_INTR_EN_M4_R {
        FFE0_6_INTR_EN_M4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FFE0_7 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_7_intr_en_m4(&self) -> FFE0_7_INTR_EN_M4_R {
        FFE0_7_INTR_EN_M4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FFE0_0 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_0_intr_en_m4(&mut self) -> FFE0_0_INTR_EN_M4_W {
        FFE0_0_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 1 - FFE0_1 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_1_intr_en_m4(&mut self) -> FFE0_1_INTR_EN_M4_W {
        FFE0_1_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 2 - FFE0_2 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_2_intr_en_m4(&mut self) -> FFE0_2_INTR_EN_M4_W {
        FFE0_2_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 3 - FFE0_3 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_3_intr_en_m4(&mut self) -> FFE0_3_INTR_EN_M4_W {
        FFE0_3_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 4 - FFE0_4 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_4_intr_en_m4(&mut self) -> FFE0_4_INTR_EN_M4_W {
        FFE0_4_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 5 - FFE0_5 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_5_intr_en_m4(&mut self) -> FFE0_5_INTR_EN_M4_W {
        FFE0_5_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 6 - FFE0_6 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_6_intr_en_m4(&mut self) -> FFE0_6_INTR_EN_M4_W {
        FFE0_6_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 7 - FFE0_7 interrupt enable for M4"]
    #[inline(always)]
    pub fn ffe0_7_intr_en_m4(&mut self) -> FFE0_7_INTR_EN_M4_W {
        FFE0_7_INTR_EN_M4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FFE0 interrupt enable for M4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_intr_en_m4](index.html) module"]
pub struct FFE_INTR_EN_M4_SPEC;
impl crate::RegisterSpec for FFE_INTR_EN_M4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_intr_en_m4::R](R) reader structure"]
impl crate::Readable for FFE_INTR_EN_M4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_intr_en_m4::W](W) writer structure"]
impl crate::Writable for FFE_INTR_EN_M4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_INTR_EN_M4 to value 0"]
impl crate::Resettable for FFE_INTR_EN_M4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
