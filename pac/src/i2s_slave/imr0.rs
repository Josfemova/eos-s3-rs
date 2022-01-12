#[doc = "Register `IMR0` reader"]
pub struct R(crate::R<IMR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR0` writer"]
pub struct W(crate::W<IMR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR0_SPEC>;
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
impl From<crate::W<IMR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Masks TX FIFO Overrun interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFOM_A {
    #[doc = "0: Unmask the interrupt"]
    UNMASK = 0,
    #[doc = "1: Mask the interrupt"]
    MASK = 1,
}
impl From<RXFOM_A> for bool {
    #[inline(always)]
    fn from(variant: RXFOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFOM` reader - Masks TX FIFO Overrun interrupt."]
pub struct RXFOM_R(crate::FieldReader<bool, RXFOM_A>);
impl RXFOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFOM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFOM_A {
        match self.bits {
            false => RXFOM_A::UNMASK,
            true => RXFOM_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == RXFOM_A::UNMASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == RXFOM_A::MASK
    }
}
impl core::ops::Deref for RXFOM_R {
    type Target = crate::FieldReader<bool, RXFOM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFOM` writer - Masks TX FIFO Overrun interrupt."]
pub struct RXFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFOM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RXFOM_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RXFOM_A::MASK)
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
#[doc = "Masks TX FIFO Empty interrupt."]
pub type TXFOM_A = RXFOM_A;
#[doc = "Field `TXFOM` reader - Masks TX FIFO Empty interrupt."]
pub type TXFOM_R = RXFOM_R;
#[doc = "Field `TXFOM` writer - Masks TX FIFO Empty interrupt."]
pub struct TXFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFOM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(TXFOM_A::UNMASK)
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TXFOM_A::MASK)
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
impl R {
    #[doc = "Bit 4 - Masks TX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rxfom(&self) -> RXFOM_R {
        RXFOM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Masks TX FIFO Empty interrupt."]
    #[inline(always)]
    pub fn txfom(&self) -> TXFOM_R {
        TXFOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Masks TX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rxfom(&mut self) -> RXFOM_W {
        RXFOM_W { w: self }
    }
    #[doc = "Bit 5 - Masks TX FIFO Empty interrupt."]
    #[inline(always)]
    pub fn txfom(&mut self) -> TXFOM_W {
        TXFOM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr0](index.html) module"]
pub struct IMR0_SPEC;
impl crate::RegisterSpec for IMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr0::R](R) reader structure"]
impl crate::Readable for IMR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr0::W](W) writer structure"]
impl crate::Writable for IMR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR0 to value 0"]
impl crate::Resettable for IMR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
