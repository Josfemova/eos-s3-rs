#[doc = "Register `UART_IMSC` reader"]
pub struct R(crate::R<UART_IMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IMSC` writer"]
pub struct W(crate::W<UART_IMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IMSC_SPEC>;
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
impl From<crate::W<UART_IMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMIMIM` reader - nUARTRI modem interrupt mask."]
pub struct RMIMIM_R(crate::FieldReader<bool, bool>);
impl RMIMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMIMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMIMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMIMIM` writer - nUARTRI modem interrupt mask."]
pub struct RMIMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIMIM_W<'a> {
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
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask."]
pub struct CTSMIM_R(crate::FieldReader<bool, bool>);
impl CTSMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask."]
pub struct CTSMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIM_W<'a> {
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
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask."]
pub struct DCDMIM_R(crate::FieldReader<bool, bool>);
impl DCDMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask."]
pub struct DCDMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIM_W<'a> {
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
#[doc = "Field `DSRMIM` reader - nUARTDSR interrupt mask."]
pub struct DSRMIM_R(crate::FieldReader<bool, bool>);
impl DSRMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMIM` writer - nUARTDSR interrupt mask."]
pub struct DSRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIM_W<'a> {
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
#[doc = "Field `RXIM` reader - Receive interrupt mask."]
pub struct RXIM_R(crate::FieldReader<bool, bool>);
impl RXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIM` writer - Receive interrupt mask."]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
#[doc = "Field `TXMIM` reader - Transmit interrupt mask."]
pub struct TXMIM_R(crate::FieldReader<bool, bool>);
impl TXMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIM` writer - Transmit interrupt mask."]
pub struct TXMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIM_W<'a> {
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
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask."]
pub struct RTIM_R(crate::FieldReader<bool, bool>);
impl RTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask."]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
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
#[doc = "Field `FEIM` reader - Framing error interrupt mask."]
pub struct FEIM_R(crate::FieldReader<bool, bool>);
impl FEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIM` writer - Framing error interrupt mask."]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
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
#[doc = "Field `PEIM` reader - Parity error interrupt mask."]
pub struct PEIM_R(crate::FieldReader<bool, bool>);
impl PEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIM` writer - Parity error interrupt mask."]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
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
#[doc = "Field `BEIM` reader - Break error interrupt mask."]
pub struct BEIM_R(crate::FieldReader<bool, bool>);
impl BEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIM` writer - Break error interrupt mask."]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
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
#[doc = "Field `OEIM` reader - Overrun error interrupt mask."]
pub struct OEIM_R(crate::FieldReader<bool, bool>);
impl OEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEIM` writer - Overrun error interrupt mask."]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask."]
    #[inline(always)]
    pub fn rmimim(&self) -> RMIMIM_R {
        RMIMIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask."]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR interrupt mask."]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask."]
    #[inline(always)]
    pub fn txmim(&self) -> TXMIM_R {
        TXMIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt mask."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - nUARTRI modem interrupt mask."]
    #[inline(always)]
    pub fn rmimim(&mut self) -> RMIMIM_W {
        RMIMIM_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask."]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CTSMIM_W {
        CTSMIM_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask."]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DCDMIM_W {
        DCDMIM_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR interrupt mask."]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DSRMIM_W {
        DSRMIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt mask."]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt mask."]
    #[inline(always)]
    pub fn txmim(&mut self) -> TXMIM_W {
        TXMIM_W { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask."]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt mask."]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt mask."]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt mask."]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 10 - Overrun error interrupt mask."]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Set/Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_imsc](index.html) module"]
pub struct UART_IMSC_SPEC;
impl crate::RegisterSpec for UART_IMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_imsc::R](R) reader structure"]
impl crate::Readable for UART_IMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_imsc::W](W) writer structure"]
impl crate::Writable for UART_IMSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IMSC to value 0"]
impl crate::Resettable for UART_IMSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
