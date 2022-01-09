#[doc = "Register `UART_IFLS` reader"]
pub struct R(crate::R<UART_IFLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IFLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IFLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IFLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IFLS` writer"]
pub struct W(crate::W<UART_IFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IFLS_SPEC>;
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
impl From<crate::W<UART_IFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IFLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit interrupt FIFO level select. The trigger points for the receive interrupt are as follows\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXIFLSEL_A {
    #[doc = "0: Trigger when corresponding FIFO becomes ≤ 1/8 full"]
    ONE_EIGHT = 0,
    #[doc = "1: Trigger when corresponding FIFO becomes ≤ 1/4 full"]
    ONE_QUARTER = 1,
    #[doc = "2: Trigger when corresponding FIFO becomes ≤ 1/2 full"]
    ONE_HALF = 2,
    #[doc = "3: Trigger when corresponding FIFO becomes ≤ 3/4 full"]
    THREE_QUARTERS = 3,
    #[doc = "4: Trigger when corresponding FIFO becomes ≤ 7/8 full"]
    SEVEN_EIGHTS = 4,
}
impl From<TXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXIFLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXIFLSEL` reader - Transmit interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
pub struct TXIFLSEL_R(crate::FieldReader<u8, TXIFLSEL_A>);
impl TXIFLSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXIFLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXIFLSEL_A> {
        match self.bits {
            0 => Some(TXIFLSEL_A::ONE_EIGHT),
            1 => Some(TXIFLSEL_A::ONE_QUARTER),
            2 => Some(TXIFLSEL_A::ONE_HALF),
            3 => Some(TXIFLSEL_A::THREE_QUARTERS),
            4 => Some(TXIFLSEL_A::SEVEN_EIGHTS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_EIGHT`"]
    #[inline(always)]
    pub fn is_one_eight(&self) -> bool {
        **self == TXIFLSEL_A::ONE_EIGHT
    }
    #[doc = "Checks if the value of the field is `ONE_QUARTER`"]
    #[inline(always)]
    pub fn is_one_quarter(&self) -> bool {
        **self == TXIFLSEL_A::ONE_QUARTER
    }
    #[doc = "Checks if the value of the field is `ONE_HALF`"]
    #[inline(always)]
    pub fn is_one_half(&self) -> bool {
        **self == TXIFLSEL_A::ONE_HALF
    }
    #[doc = "Checks if the value of the field is `THREE_QUARTERS`"]
    #[inline(always)]
    pub fn is_three_quarters(&self) -> bool {
        **self == TXIFLSEL_A::THREE_QUARTERS
    }
    #[doc = "Checks if the value of the field is `SEVEN_EIGHTS`"]
    #[inline(always)]
    pub fn is_seven_eights(&self) -> bool {
        **self == TXIFLSEL_A::SEVEN_EIGHTS
    }
}
impl core::ops::Deref for TXIFLSEL_R {
    type Target = crate::FieldReader<u8, TXIFLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIFLSEL` writer - Transmit interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 1/8 full"]
    #[inline(always)]
    pub fn one_eight(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::ONE_EIGHT)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 1/4 full"]
    #[inline(always)]
    pub fn one_quarter(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::ONE_QUARTER)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 1/2 full"]
    #[inline(always)]
    pub fn one_half(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::ONE_HALF)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 3/4 full"]
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::THREE_QUARTERS)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 7/8 full"]
    #[inline(always)]
    pub fn seven_eights(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::SEVEN_EIGHTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
pub type RXIFLSEL_A = TXIFLSEL_A;
#[doc = "Field `RXIFLSEL` reader - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
pub type RXIFLSEL_R = TXIFLSEL_R;
#[doc = "Field `RXIFLSEL` writer - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIFLSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 1/8 full"]
    #[inline(always)]
    pub fn one_eight(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::ONE_EIGHT)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 1/4 full"]
    #[inline(always)]
    pub fn one_quarter(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::ONE_QUARTER)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 1/2 full"]
    #[inline(always)]
    pub fn one_half(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::ONE_HALF)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 3/4 full"]
    #[inline(always)]
    pub fn three_quarters(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::THREE_QUARTERS)
    }
    #[doc = "Trigger when corresponding FIFO becomes ≤ 7/8 full"]
    #[inline(always)]
    pub fn seven_eights(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::SEVEN_EIGHTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ifls](index.html) module"]
pub struct UART_IFLS_SPEC;
impl crate::RegisterSpec for UART_IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ifls::R](R) reader structure"]
impl crate::Readable for UART_IFLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ifls::W](W) writer structure"]
impl crate::Writable for UART_IFLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IFLS to value 0x12"]
impl crate::Resettable for UART_IFLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x12
    }
}
