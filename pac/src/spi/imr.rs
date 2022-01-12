#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit FIFO Empty Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIM_A {
    #[doc = "0: Mask the interrupt"]
    MASK = 0,
    #[doc = "1: Unmask the interrupt"]
    UNMASK = 1,
}
impl From<TXEIM_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIM` reader - Transmit FIFO Empty Interrupt Mask"]
pub struct TXEIM_R(crate::FieldReader<bool, TXEIM_A>);
impl TXEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIM_A {
        match self.bits {
            false => TXEIM_A::MASK,
            true => TXEIM_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == TXEIM_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == TXEIM_A::UNMASK
    }
}
impl core::ops::Deref for TXEIM_R {
    type Target = crate::FieldReader<bool, TXEIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEIM` writer - Transmit FIFO Empty Interrupt Mask"]
pub struct TXEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TXEIM_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(TXEIM_A::UNMASK)
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
#[doc = "Transmit FIFO Overflow Interrupt Mask"]
pub type TXOIM_A = TXEIM_A;
#[doc = "Field `TXOIM` reader - Transmit FIFO Overflow Interrupt Mask"]
pub type TXOIM_R = TXEIM_R;
#[doc = "Field `TXOIM` writer - Transmit FIFO Overflow Interrupt Mask"]
pub struct TXOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TXOIM_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(TXOIM_A::UNMASK)
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
#[doc = "Receive FIFO Underflow Interrupt Mask"]
pub type RXUIM_A = TXEIM_A;
#[doc = "Field `RXUIM` reader - Receive FIFO Underflow Interrupt Mask"]
pub type RXUIM_R = TXEIM_R;
#[doc = "Field `RXUIM` writer - Receive FIFO Underflow Interrupt Mask"]
pub struct RXUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RXUIM_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RXUIM_A::UNMASK)
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
#[doc = "Receive FIFO Overflow Interrupt Mask"]
pub type RXFOIM_A = TXEIM_A;
#[doc = "Field `RXFOIM` reader - Receive FIFO Overflow Interrupt Mask"]
pub type RXFOIM_R = TXEIM_R;
#[doc = "Field `RXFOIM` writer - Receive FIFO Overflow Interrupt Mask"]
pub struct RXFOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFOIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RXFOIM_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RXFOIM_A::UNMASK)
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
#[doc = "Receive FIFO Full Interrupt Mask"]
pub type RXFIM_A = TXEIM_A;
#[doc = "Field `RXFIM` reader - Receive FIFO Full Interrupt Mask"]
pub type RXFIM_R = TXEIM_R;
#[doc = "Field `RXFIM` writer - Receive FIFO Full Interrupt Mask"]
pub struct RXFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RXFIM_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RXFIM_A::UNMASK)
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
#[doc = "Multi-Master Contention Interrupt Mask. This bit field is not present if the SPI Master is configured as a serial-slave device."]
pub type MSTIM_A = TXEIM_A;
#[doc = "Field `MSTIM` reader - Multi-Master Contention Interrupt Mask. This bit field is not present if the SPI Master is configured as a serial-slave device."]
pub type MSTIM_R = TXEIM_R;
#[doc = "Field `MSTIM` writer - Multi-Master Contention Interrupt Mask. This bit field is not present if the SPI Master is configured as a serial-slave device."]
pub struct MSTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTIM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSTIM_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(MSTIM_A::UNMASK)
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
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txeim(&self) -> TXEIM_R {
        TXEIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn txoim(&self) -> TXOIM_R {
        TXOIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Mask"]
    #[inline(always)]
    pub fn rxuim(&self) -> RXUIM_R {
        RXUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn rxfoim(&self) -> RXFOIM_R {
        RXFOIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxfim(&self) -> RXFIM_R {
        RXFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi-Master Contention Interrupt Mask. This bit field is not present if the SPI Master is configured as a serial-slave device."]
    #[inline(always)]
    pub fn mstim(&self) -> MSTIM_R {
        MSTIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txeim(&mut self) -> TXEIM_W {
        TXEIM_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn txoim(&mut self) -> TXOIM_W {
        TXOIM_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Mask"]
    #[inline(always)]
    pub fn rxuim(&mut self) -> RXUIM_W {
        RXUIM_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn rxfoim(&mut self) -> RXFOIM_W {
        RXFOIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxfim(&mut self) -> RXFIM_W {
        RXFIM_W { w: self }
    }
    #[doc = "Bit 5 - Multi-Master Contention Interrupt Mask. This bit field is not present if the SPI Master is configured as a serial-slave device."]
    #[inline(always)]
    pub fn mstim(&mut self) -> MSTIM_W {
        MSTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register: This read/write register masks or enables all interrupts generated by the SPI Master.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0x1f"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
