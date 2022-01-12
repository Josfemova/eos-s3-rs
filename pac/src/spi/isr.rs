#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEIS` reader - Transmit FIFO Empty Interrupt Status"]
pub struct TXEIS_R(crate::FieldReader<bool, bool>);
impl TXEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEIS` writer - Transmit FIFO Empty Interrupt Status"]
pub struct TXEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIS_W<'a> {
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
#[doc = "Field `TXOIS` reader - Transmit FIFO Overflow Interrupt Status"]
pub struct TXOIS_R(crate::FieldReader<bool, bool>);
impl TXOIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOIS` writer - Transmit FIFO Overflow Interrupt Status"]
pub struct TXOIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOIS_W<'a> {
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
#[doc = "Field `RXUIS` reader - Receive FIFO Underflow Interrupt Status"]
pub struct RXUIS_R(crate::FieldReader<bool, bool>);
impl RXUIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUIS` writer - Receive FIFO Underflow Interrupt Status"]
pub struct RXUIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUIS_W<'a> {
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
#[doc = "Field `RXOIS` reader - Receive FIFO Overflow Interrupt Status"]
pub struct RXOIS_R(crate::FieldReader<bool, bool>);
impl RXOIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIS` writer - Receive FIFO Overflow Interrupt Status"]
pub struct RXOIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIS_W<'a> {
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
#[doc = "Field `RXFIS` reader - Receive FIFO Full Interrupt Status"]
pub struct RXFIS_R(crate::FieldReader<bool, bool>);
impl RXFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIS` writer - Receive FIFO Full Interrupt Status"]
pub struct RXFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIS_W<'a> {
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
#[doc = "Field `MSTIS` reader - Multi-Master Contention Interrupt Status. This bit field is not present if the SPI Master is configured as a serial-slave device."]
pub struct MSTIS_R(crate::FieldReader<bool, bool>);
impl MSTIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTIS` writer - Multi-Master Contention Interrupt Status. This bit field is not present if the SPI Master is configured as a serial-slave device."]
pub struct MSTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIS_W<'a> {
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
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Status"]
    #[inline(always)]
    pub fn txeis(&self) -> TXEIS_R {
        TXEIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn txois(&self) -> TXOIS_R {
        TXOIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Status"]
    #[inline(always)]
    pub fn rxuis(&self) -> RXUIS_R {
        RXUIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rxois(&self) -> RXOIS_R {
        RXOIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn rxfis(&self) -> RXFIS_R {
        RXFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi-Master Contention Interrupt Status. This bit field is not present if the SPI Master is configured as a serial-slave device."]
    #[inline(always)]
    pub fn mstis(&self) -> MSTIS_R {
        MSTIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO Empty Interrupt Status"]
    #[inline(always)]
    pub fn txeis(&mut self) -> TXEIS_W {
        TXEIS_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn txois(&mut self) -> TXOIS_W {
        TXOIS_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO Underflow Interrupt Status"]
    #[inline(always)]
    pub fn rxuis(&mut self) -> RXUIS_W {
        RXUIS_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rxois(&mut self) -> RXOIS_W {
        RXOIS_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn rxfis(&mut self) -> RXFIS_W {
        RXFIS_W { w: self }
    }
    #[doc = "Bit 5 - Multi-Master Contention Interrupt Status. This bit field is not present if the SPI Master is configured as a serial-slave device."]
    #[inline(always)]
    pub fn mstis(&mut self) -> MSTIS_W {
        MSTIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register: This register reports the status of SPI Master interrupts after they have been masked.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
