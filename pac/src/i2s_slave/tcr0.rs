#[doc = "Register `TCR0` reader"]
pub struct R(crate::R<TCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR0` writer"]
pub struct W(crate::W<TCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR0_SPEC>;
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
impl From<crate::W<TCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "These bits are used to program the data resolution of the transmitter and ensures the MSB of the data is transmitted first. Programmed resolution must be less than or equal to `I2S_TX_WORDSIZE_x`. If the selected resolution is greater than `I2S_TX_WORDSIZE_x`, the transmit channel defaults back to `I2S_TX_WORDSIZE_RESET_x` value. The channel must be disabled prior to any changes in this value (`TERx\\[0\\]
= 0`).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WLEN_A {
    #[doc = "0: Ignore word length."]
    IGNORE_WORD_LEN = 0,
    #[doc = "1: 12 bit resolution."]
    WORD_12BIT = 1,
    #[doc = "2: 16 bit resolution."]
    WORD_16BIT = 2,
    #[doc = "3: 20 bit resolution."]
    WORD_20BIT = 3,
    #[doc = "4: 24 bit resolution."]
    WORD_24BIT = 4,
    #[doc = "5: 32 bit resolution."]
    WORD_32BIT = 5,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WLEN` reader - These bits are used to program the data resolution of the transmitter and ensures the MSB of the data is transmitted first. Programmed resolution must be less than or equal to `I2S_TX_WORDSIZE_x`. If the selected resolution is greater than `I2S_TX_WORDSIZE_x`, the transmit channel defaults back to `I2S_TX_WORDSIZE_RESET_x` value. The channel must be disabled prior to any changes in this value (`TERx\\[0\\]
= 0`)."]
pub struct WLEN_R(crate::FieldReader<u8, WLEN_A>);
impl WLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WLEN_A> {
        match self.bits {
            0 => Some(WLEN_A::IGNORE_WORD_LEN),
            1 => Some(WLEN_A::WORD_12BIT),
            2 => Some(WLEN_A::WORD_16BIT),
            3 => Some(WLEN_A::WORD_20BIT),
            4 => Some(WLEN_A::WORD_24BIT),
            5 => Some(WLEN_A::WORD_32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE_WORD_LEN`"]
    #[inline(always)]
    pub fn is_ignore_word_len(&self) -> bool {
        **self == WLEN_A::IGNORE_WORD_LEN
    }
    #[doc = "Checks if the value of the field is `WORD_12BIT`"]
    #[inline(always)]
    pub fn is_word_12bit(&self) -> bool {
        **self == WLEN_A::WORD_12BIT
    }
    #[doc = "Checks if the value of the field is `WORD_16BIT`"]
    #[inline(always)]
    pub fn is_word_16bit(&self) -> bool {
        **self == WLEN_A::WORD_16BIT
    }
    #[doc = "Checks if the value of the field is `WORD_20BIT`"]
    #[inline(always)]
    pub fn is_word_20bit(&self) -> bool {
        **self == WLEN_A::WORD_20BIT
    }
    #[doc = "Checks if the value of the field is `WORD_24BIT`"]
    #[inline(always)]
    pub fn is_word_24bit(&self) -> bool {
        **self == WLEN_A::WORD_24BIT
    }
    #[doc = "Checks if the value of the field is `WORD_32BIT`"]
    #[inline(always)]
    pub fn is_word_32bit(&self) -> bool {
        **self == WLEN_A::WORD_32BIT
    }
}
impl core::ops::Deref for WLEN_R {
    type Target = crate::FieldReader<u8, WLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLEN` writer - These bits are used to program the data resolution of the transmitter and ensures the MSB of the data is transmitted first. Programmed resolution must be less than or equal to `I2S_TX_WORDSIZE_x`. If the selected resolution is greater than `I2S_TX_WORDSIZE_x`, the transmit channel defaults back to `I2S_TX_WORDSIZE_RESET_x` value. The channel must be disabled prior to any changes in this value (`TERx\\[0\\]
= 0`)."]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Ignore word length."]
    #[inline(always)]
    pub fn ignore_word_len(self) -> &'a mut W {
        self.variant(WLEN_A::IGNORE_WORD_LEN)
    }
    #[doc = "12 bit resolution."]
    #[inline(always)]
    pub fn word_12bit(self) -> &'a mut W {
        self.variant(WLEN_A::WORD_12BIT)
    }
    #[doc = "16 bit resolution."]
    #[inline(always)]
    pub fn word_16bit(self) -> &'a mut W {
        self.variant(WLEN_A::WORD_16BIT)
    }
    #[doc = "20 bit resolution."]
    #[inline(always)]
    pub fn word_20bit(self) -> &'a mut W {
        self.variant(WLEN_A::WORD_20BIT)
    }
    #[doc = "24 bit resolution."]
    #[inline(always)]
    pub fn word_24bit(self) -> &'a mut W {
        self.variant(WLEN_A::WORD_24BIT)
    }
    #[doc = "32 bit resolution."]
    #[inline(always)]
    pub fn word_32bit(self) -> &'a mut W {
        self.variant(WLEN_A::WORD_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - These bits are used to program the data resolution of the transmitter and ensures the MSB of the data is transmitted first. Programmed resolution must be less than or equal to `I2S_TX_WORDSIZE_x`. If the selected resolution is greater than `I2S_TX_WORDSIZE_x`, the transmit channel defaults back to `I2S_TX_WORDSIZE_RESET_x` value. The channel must be disabled prior to any changes in this value (`TERx\\[0\\]
= 0`)."]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits are used to program the data resolution of the transmitter and ensures the MSB of the data is transmitted first. Programmed resolution must be less than or equal to `I2S_TX_WORDSIZE_x`. If the selected resolution is greater than `I2S_TX_WORDSIZE_x`, the transmit channel defaults back to `I2S_TX_WORDSIZE_RESET_x` value. The channel must be disabled prior to any changes in this value (`TERx\\[0\\]
= 0`)."]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr0](index.html) module"]
pub struct TCR0_SPEC;
impl crate::RegisterSpec for TCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr0::R](R) reader structure"]
impl crate::Readable for TCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr0::W](W) writer structure"]
impl crate::Writable for TCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR0 to value 0x02"]
impl crate::Resettable for TCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
