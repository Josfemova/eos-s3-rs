#[doc = "Register `UART_ITIP` reader"]
pub struct R(crate::R<UART_ITIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_ITIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_ITIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_ITIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_ITIP` writer"]
pub struct W(crate::W<UART_ITIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ITIP_SPEC>;
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
impl From<crate::W<UART_ITIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_ITIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UARTRXD` reader - "]
pub struct UARTRXD_R(crate::FieldReader<bool, bool>);
impl UARTRXD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UARTRXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTRXD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRIN` reader - "]
pub struct SIRIN_R(crate::FieldReader<bool, bool>);
impl SIRIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIRIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIRIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nUARTDSR` reader - "]
pub struct NUARTDSR_R(crate::FieldReader<bool, bool>);
impl NUARTDSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NUARTDSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUARTDSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nUARTCTS` reader - "]
pub struct NUARTCTS_R(crate::FieldReader<bool, bool>);
impl NUARTCTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NUARTCTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUARTCTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nUARTDCD` reader - "]
pub struct NUARTDCD_R(crate::FieldReader<bool, bool>);
impl NUARTDCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NUARTDCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUARTDCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `nUARTRI` reader - "]
pub struct NUARTRI_R(crate::FieldReader<bool, bool>);
impl NUARTRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NUARTRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUARTRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTRXDMACLR` reader - "]
pub struct UARTRXDMACLR_R(crate::FieldReader<bool, bool>);
impl UARTRXDMACLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UARTRXDMACLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTRXDMACLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTRXDMACLR` writer - "]
pub struct UARTRXDMACLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTRXDMACLR_W<'a> {
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
#[doc = "Field `UARTTXDMACLR` reader - "]
pub struct UARTTXDMACLR_R(crate::FieldReader<bool, bool>);
impl UARTTXDMACLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UARTTXDMACLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UARTTXDMACLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTTXDMACLR` writer - "]
pub struct UARTTXDMACLR_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTTXDMACLR_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uartrxd(&self) -> UARTRXD_R {
        UARTRXD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sirin(&self) -> SIRIN_R {
        SIRIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn n_uartdsr(&self) -> NUARTDSR_R {
        NUARTDSR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn n_uartcts(&self) -> NUARTCTS_R {
        NUARTCTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn n_uartdcd(&self) -> NUARTDCD_R {
        NUARTDCD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn n_uartri(&self) -> NUARTRI_R {
        NUARTRI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uartrxdmaclr(&self) -> UARTRXDMACLR_R {
        UARTRXDMACLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uarttxdmaclr(&self) -> UARTTXDMACLR_R {
        UARTTXDMACLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uartrxdmaclr(&mut self) -> UARTRXDMACLR_W {
        UARTRXDMACLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uarttxdmaclr(&mut self) -> UARTTXDMACLR_W {
        UARTTXDMACLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration test input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_itip](index.html) module"]
pub struct UART_ITIP_SPEC;
impl crate::RegisterSpec for UART_ITIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_itip::R](R) reader structure"]
impl crate::Readable for UART_ITIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_itip::W](W) writer structure"]
impl crate::Writable for UART_ITIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_ITIP to value 0"]
impl crate::Resettable for UART_ITIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
