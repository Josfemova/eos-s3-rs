#[doc = "Register `DMA_INTR_MASK` reader"]
pub struct R(crate::R<DMA_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INTR_MASK` writer"]
pub struct W(crate::W<DMA_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INTR_MASK_SPEC>;
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
impl From<crate::W<DMA_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "1: disable interrupt, 0:enable interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_HERROR_MASK_A {
    #[doc = "0: Mask the interrupt"]
    MASK = 0,
    #[doc = "1: Unmask the interrupt"]
    UNMASK = 1,
}
impl From<DMA_HERROR_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_HERROR_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dma_herror_mask` reader - 1: disable interrupt, 0:enable interrupt"]
pub struct DMA_HERROR_MASK_R(crate::FieldReader<bool, DMA_HERROR_MASK_A>);
impl DMA_HERROR_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_HERROR_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_HERROR_MASK_A {
        match self.bits {
            false => DMA_HERROR_MASK_A::MASK,
            true => DMA_HERROR_MASK_A::UNMASK,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == DMA_HERROR_MASK_A::MASK
    }
    #[doc = "Checks if the value of the field is `UNMASK`"]
    #[inline(always)]
    pub fn is_unmask(&self) -> bool {
        **self == DMA_HERROR_MASK_A::UNMASK
    }
}
impl core::ops::Deref for DMA_HERROR_MASK_R {
    type Target = crate::FieldReader<bool, DMA_HERROR_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_herror_mask` writer - 1: disable interrupt, 0:enable interrupt"]
pub struct DMA_HERROR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_HERROR_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_HERROR_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(DMA_HERROR_MASK_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(DMA_HERROR_MASK_A::UNMASK)
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
#[doc = "1: mask rx data available, 0:don't mask"]
pub type RX_DATA_AVAILABLE_MASK_A = DMA_HERROR_MASK_A;
#[doc = "Field `rx_data_available_mask` reader - 1: mask rx data available, 0:don't mask"]
pub type RX_DATA_AVAILABLE_MASK_R = DMA_HERROR_MASK_R;
#[doc = "Field `rx_data_available_mask` writer - 1: mask rx data available, 0:don't mask"]
pub struct RX_DATA_AVAILABLE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_AVAILABLE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DATA_AVAILABLE_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(RX_DATA_AVAILABLE_MASK_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(RX_DATA_AVAILABLE_MASK_A::UNMASK)
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
#[doc = "1: Mask the ahb FIFO bridge overflow, 0: interrupts are enabled"]
pub type AHB_BRIDGE_FIFO_OVERFLOW_MASK_A = DMA_HERROR_MASK_A;
#[doc = "Field `ahb_bridge_fifo_overflow_mask` reader - 1: Mask the ahb FIFO bridge overflow, 0: interrupts are enabled"]
pub type AHB_BRIDGE_FIFO_OVERFLOW_MASK_R = DMA_HERROR_MASK_R;
#[doc = "Field `ahb_bridge_fifo_overflow_mask` writer - 1: Mask the ahb FIFO bridge overflow, 0: interrupts are enabled"]
pub struct AHB_BRIDGE_FIFO_OVERFLOW_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_BRIDGE_FIFO_OVERFLOW_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(
        self,
        variant: AHB_BRIDGE_FIFO_OVERFLOW_MASK_A,
    ) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AHB_BRIDGE_FIFO_OVERFLOW_MASK_A::MASK)
    }
    #[doc = "Unmask the interrupt"]
    #[inline(always)]
    pub fn unmask(self) -> &'a mut W {
        self.variant(AHB_BRIDGE_FIFO_OVERFLOW_MASK_A::UNMASK)
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
impl R {
    #[doc = "Bit 0 - 1: disable interrupt, 0:enable interrupt"]
    #[inline(always)]
    pub fn dma_herror_mask(&self) -> DMA_HERROR_MASK_R {
        DMA_HERROR_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: mask rx data available, 0:don't mask"]
    #[inline(always)]
    pub fn rx_data_available_mask(&self) -> RX_DATA_AVAILABLE_MASK_R {
        RX_DATA_AVAILABLE_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Mask the ahb FIFO bridge overflow, 0: interrupts are enabled"]
    #[inline(always)]
    pub fn ahb_bridge_fifo_overflow_mask(
        &self,
    ) -> AHB_BRIDGE_FIFO_OVERFLOW_MASK_R {
        AHB_BRIDGE_FIFO_OVERFLOW_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: disable interrupt, 0:enable interrupt"]
    #[inline(always)]
    pub fn dma_herror_mask(&mut self) -> DMA_HERROR_MASK_W {
        DMA_HERROR_MASK_W { w: self }
    }
    #[doc = "Bit 1 - 1: mask rx data available, 0:don't mask"]
    #[inline(always)]
    pub fn rx_data_available_mask(&mut self) -> RX_DATA_AVAILABLE_MASK_W {
        RX_DATA_AVAILABLE_MASK_W { w: self }
    }
    #[doc = "Bit 2 - 1: Mask the ahb FIFO bridge overflow, 0: interrupts are enabled"]
    #[inline(always)]
    pub fn ahb_bridge_fifo_overflow_mask(
        &mut self,
    ) -> AHB_BRIDGE_FIFO_OVERFLOW_MASK_W {
        AHB_BRIDGE_FIFO_OVERFLOW_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_intr_mask](index.html) module"]
pub struct DMA_INTR_MASK_SPEC;
impl crate::RegisterSpec for DMA_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_intr_mask::R](R) reader structure"]
impl crate::Readable for DMA_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_intr_mask::W](W) writer structure"]
impl crate::Writable for DMA_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INTR_MASK to value 0"]
impl crate::Resettable for DMA_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
