#[doc = "Register `UART_ICR` writer"]
pub struct W(crate::W<UART_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_ICR_SPEC>;
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
impl From<crate::W<UART_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMIC` writer - nUARTRI modem interrupt clear. Writing a 1 clears the interrupt."]
pub struct RIMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIC_W<'a> {
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
#[doc = "Field `CTSMIC` writer - nUARTCTS modem interrupt clear. Writing a 1 clears the interrupt."]
pub struct CTSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIC_W<'a> {
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
#[doc = "Field `DCDMIC` writer - nUARTDCD modem interrupt clear. Writing a 1 clears the interrupt."]
pub struct DCDMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIC_W<'a> {
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
#[doc = "Field `DSRMIC` writer - nUARTDSR modem interrupt clear. Writing a 1 clears the interrupt."]
pub struct DSRMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIC_W<'a> {
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
#[doc = "Field `RXIC` writer - Receive interrupt clear. Writing a 1 clears the interrupt."]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Field `TXIC` writer - Transmit interrupt clear. Writing a 1 clears the interrupt."]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Field `RTIC` writer - Receive timeout interrupt clear. Writing a 1 clears the interrupt."]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
#[doc = "Field `FEIC` writer - Framing error interrupt clear. Writing a 1 clears the interrupt."]
pub struct FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIC_W<'a> {
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
#[doc = "Field `PEIC` writer - Parity error interrupt clear. Writing a 1 clears the interrupt."]
pub struct PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIC_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BEIC` writer - Break error interrupt clear. Writing a 1 clears the interrupt."]
pub struct BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIC_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `OEIC` writer - Overrun error interrupt clear. Writing a 1 clears the interrupt."]
pub struct OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIC_W<'a> {
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
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn rimic(&mut self) -> RIMIC_W {
        RIMIC_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CTSMIC_W {
        CTSMIC_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DCDMIC_W {
        DCDMIC_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn dsrmic(&mut self) -> DSRMIC_W {
        DSRMIC_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W { w: self }
    }
    #[doc = "Bit 10 - Overrun error interrupt clear. Writing a 1 clears the interrupt."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_icr](index.html) module"]
pub struct UART_ICR_SPEC;
impl crate::RegisterSpec for UART_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_icr::W](W) writer structure"]
impl crate::Writable for UART_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_ICR to value 0"]
impl crate::Resettable for UART_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
