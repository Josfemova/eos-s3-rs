#[doc = "Register `ISR0` reader"]
pub struct R(crate::R<ISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Status of Transmit Empty Trigger interrupt. TX FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "0: TX FIFO write valid."]
    WRITE_VALID = 0,
    #[doc = "1: TX FIFO write overrun."]
    WRITE_OVERRUN = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - Status of Transmit Empty Trigger interrupt. TX FIFO is empty."]
pub struct TXFE_R(crate::FieldReader<bool, TXFE_A>);
impl TXFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::WRITE_VALID,
            true => TXFE_A::WRITE_OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_VALID`"]
    #[inline(always)]
    pub fn is_write_valid(&self) -> bool {
        **self == TXFE_A::WRITE_VALID
    }
    #[doc = "Checks if the value of the field is `WRITE_OVERRUN`"]
    #[inline(always)]
    pub fn is_write_overrun(&self) -> bool {
        **self == TXFE_A::WRITE_OVERRUN
    }
}
impl core::ops::Deref for TXFE_R {
    type Target = crate::FieldReader<bool, TXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Status of Data Overrun interrupt for the TX channel. Attempt to write to full TX FIFO. Dependencies: I2S_TX_CHANNELS > x, where x is the number of transmit channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFO_A {
    #[doc = "0: Trigger level not reached."]
    TRIGGER_NOT_REACHED = 0,
    #[doc = "1: Trigger level reached."]
    TRIGGER_REACHED = 1,
}
impl From<TXFO_A> for bool {
    #[inline(always)]
    fn from(variant: TXFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFO` reader - Status of Data Overrun interrupt for the TX channel. Attempt to write to full TX FIFO. Dependencies: I2S_TX_CHANNELS > x, where x is the number of transmit channel."]
pub struct TXFO_R(crate::FieldReader<bool, TXFO_A>);
impl TXFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFO_A {
        match self.bits {
            false => TXFO_A::TRIGGER_NOT_REACHED,
            true => TXFO_A::TRIGGER_REACHED,
        }
    }
    #[doc = "Checks if the value of the field is `TRIGGER_NOT_REACHED`"]
    #[inline(always)]
    pub fn is_trigger_not_reached(&self) -> bool {
        **self == TXFO_A::TRIGGER_NOT_REACHED
    }
    #[doc = "Checks if the value of the field is `TRIGGER_REACHED`"]
    #[inline(always)]
    pub fn is_trigger_reached(&self) -> bool {
        **self == TXFO_A::TRIGGER_REACHED
    }
}
impl core::ops::Deref for TXFO_R {
    type Target = crate::FieldReader<bool, TXFO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Status of Transmit Empty Trigger interrupt. TX FIFO is empty."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Data Overrun interrupt for the TX channel. Attempt to write to full TX FIFO. Dependencies: I2S_TX_CHANNELS > x, where x is the number of transmit channel."]
    #[inline(always)]
    pub fn txfo(&self) -> TXFO_R {
        TXFO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr0](index.html) module"]
pub struct ISR0_SPEC;
impl crate::RegisterSpec for ISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr0::R](R) reader structure"]
impl crate::Readable for ISR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR0 to value 0"]
impl crate::Resettable for ISR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
