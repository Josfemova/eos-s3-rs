#[doc = "Register `DR0` reader"]
pub struct R(crate::R<DR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR0` writer"]
pub struct W(crate::W<DR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR0_SPEC>;
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
impl From<crate::W<DR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR` reader - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read = Receive FIFO buffer (SSI_RX_FIFO_DEPTH = 0x8) Write = Transmit FIFO buffer (SSI_TX_FIFO_DEPTH = 0x83)"]
pub struct DR_R(crate::FieldReader<u16, u16>);
impl DR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR` writer - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read = Receive FIFO buffer (SSI_RX_FIFO_DEPTH = 0x8) Write = Transmit FIFO buffer (SSI_TX_FIFO_DEPTH = 0x83)"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read = Receive FIFO buffer (SSI_RX_FIFO_DEPTH = 0x8) Write = Transmit FIFO buffer (SSI_TX_FIFO_DEPTH = 0x83)"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register. When writing to this register, you must right-justify the data. Read data are automatically right-justified. Read = Receive FIFO buffer (SSI_RX_FIFO_DEPTH = 0x8) Write = Transmit FIFO buffer (SSI_TX_FIFO_DEPTH = 0x83)"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SPI Master data register is a 16-bit read/write buffer for the transmit/receive FIFOs. When the register is read, data in the receive FIFO buffer is accessed. When it is written to, data are moved into the transmit FIFO buffer; a write can occur only when SSI_EN = 1. FIFOs are reset when SSI_EN = 0. Please refer to SSIENR register (0x008) to enable and disable the SPI Master. The DR register in the SPI Master occupies 131(for TX)/8(for RX) 32-bit address locations of the memory map to facilitate AHB burst transfers. Writing to any of these address locations has the same effect as pushing the data from the pwdata bus into the transmit FIFO. Reading from any of these locations has the same effect as popping data from the receive FIFO onto the prdata bus. The FIFO buffers on the SPI Master are not addressable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr0](index.html) module"]
pub struct DR0_SPEC;
impl crate::RegisterSpec for DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr0::R](R) reader structure"]
impl crate::Readable for DR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr0::W](W) writer structure"]
impl crate::Writable for DR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for DR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
