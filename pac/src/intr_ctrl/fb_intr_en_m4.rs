#[doc = "Register `FB_INTR_EN_M4` reader"]
pub struct R(crate::R<FB_INTR_EN_M4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_INTR_EN_M4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_INTR_EN_M4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_INTR_EN_M4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB_INTR_EN_M4` writer"]
pub struct W(crate::W<FB_INTR_EN_M4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_INTR_EN_M4_SPEC>;
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
impl From<crate::W<FB_INTR_EN_M4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_INTR_EN_M4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FB_0 interrupt enable for M4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB_0_INTR_EN_M4_A {
    #[doc = "0: Disable the interrupt for the power domain"]
    ENABLE = 0,
    #[doc = "1: Enable the interrupt for the power domain"]
    DISABLE = 1,
}
impl From<FB_0_INTR_EN_M4_A> for bool {
    #[inline(always)]
    fn from(variant: FB_0_INTR_EN_M4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FB_0_INTR_EN_M4` reader - FB_0 interrupt enable for M4"]
pub struct FB_0_INTR_EN_M4_R(crate::FieldReader<bool, FB_0_INTR_EN_M4_A>);
impl FB_0_INTR_EN_M4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_0_INTR_EN_M4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB_0_INTR_EN_M4_A {
        match self.bits {
            false => FB_0_INTR_EN_M4_A::ENABLE,
            true => FB_0_INTR_EN_M4_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FB_0_INTR_EN_M4_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FB_0_INTR_EN_M4_A::DISABLE
    }
}
impl core::ops::Deref for FB_0_INTR_EN_M4_R {
    type Target = crate::FieldReader<bool, FB_0_INTR_EN_M4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_0_INTR_EN_M4` writer - FB_0 interrupt enable for M4"]
pub struct FB_0_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_0_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_0_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FB_0_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FB_0_INTR_EN_M4_A::DISABLE)
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
#[doc = "FB_1 interrupt enable for M4"]
pub type FB_1_INTR_EN_M4_A = FB_0_INTR_EN_M4_A;
#[doc = "Field `FB_1_INTR_EN_M4` reader - FB_1 interrupt enable for M4"]
pub type FB_1_INTR_EN_M4_R = FB_0_INTR_EN_M4_R;
#[doc = "Field `FB_1_INTR_EN_M4` writer - FB_1 interrupt enable for M4"]
pub struct FB_1_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_1_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_1_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FB_1_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FB_1_INTR_EN_M4_A::DISABLE)
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
#[doc = "FB_2 interrupt enable for M4"]
pub type FB_2_INTR_EN_M4_A = FB_0_INTR_EN_M4_A;
#[doc = "Field `FB_2_INTR_EN_M4` reader - FB_2 interrupt enable for M4"]
pub type FB_2_INTR_EN_M4_R = FB_0_INTR_EN_M4_R;
#[doc = "Field `FB_2_INTR_EN_M4` writer - FB_2 interrupt enable for M4"]
pub struct FB_2_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_2_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_2_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FB_2_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FB_2_INTR_EN_M4_A::DISABLE)
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
#[doc = "FB_3 interrupt enable for M4"]
pub type FB_3_INTR_EN_M4_A = FB_0_INTR_EN_M4_A;
#[doc = "Field `FB_3_INTR_EN_M4` reader - FB_3 interrupt enable for M4"]
pub type FB_3_INTR_EN_M4_R = FB_0_INTR_EN_M4_R;
#[doc = "Field `FB_3_INTR_EN_M4` writer - FB_3 interrupt enable for M4"]
pub struct FB_3_INTR_EN_M4_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_3_INTR_EN_M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB_3_INTR_EN_M4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FB_3_INTR_EN_M4_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FB_3_INTR_EN_M4_A::DISABLE)
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
impl R {
    #[doc = "Bit 0 - FB_0 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_0_intr_en_m4(&self) -> FB_0_INTR_EN_M4_R {
        FB_0_INTR_EN_M4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FB_1 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_1_intr_en_m4(&self) -> FB_1_INTR_EN_M4_R {
        FB_1_INTR_EN_M4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FB_2 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_2_intr_en_m4(&self) -> FB_2_INTR_EN_M4_R {
        FB_2_INTR_EN_M4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FB_3 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_3_intr_en_m4(&self) -> FB_3_INTR_EN_M4_R {
        FB_3_INTR_EN_M4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FB_0 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_0_intr_en_m4(&mut self) -> FB_0_INTR_EN_M4_W {
        FB_0_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 1 - FB_1 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_1_intr_en_m4(&mut self) -> FB_1_INTR_EN_M4_W {
        FB_1_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 2 - FB_2 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_2_intr_en_m4(&mut self) -> FB_2_INTR_EN_M4_W {
        FB_2_INTR_EN_M4_W { w: self }
    }
    #[doc = "Bit 3 - FB_3 interrupt enable for M4"]
    #[inline(always)]
    pub fn fb_3_intr_en_m4(&mut self) -> FB_3_INTR_EN_M4_W {
        FB_3_INTR_EN_M4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FB interrupt enable for M4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_intr_en_m4](index.html) module"]
pub struct FB_INTR_EN_M4_SPEC;
impl crate::RegisterSpec for FB_INTR_EN_M4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_intr_en_m4::R](R) reader structure"]
impl crate::Readable for FB_INTR_EN_M4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb_intr_en_m4::W](W) writer structure"]
impl crate::Writable for FB_INTR_EN_M4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FB_INTR_EN_M4 to value 0"]
impl crate::Resettable for FB_INTR_EN_M4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
