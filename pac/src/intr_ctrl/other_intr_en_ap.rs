#[doc = "Register `OTHER_INTR_EN_AP` reader"]
pub struct R(crate::R<OTHER_INTR_EN_AP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTHER_INTR_EN_AP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTHER_INTR_EN_AP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTHER_INTR_EN_AP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTHER_INTR_EN_AP` writer"]
pub struct W(crate::W<OTHER_INTR_EN_AP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTHER_INTR_EN_AP_SPEC>;
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
impl From<crate::W<OTHER_INTR_EN_AP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTHER_INTR_EN_AP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "M4 SRAM (access during low power) interrupt enable for AP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4_SRAM_INTR_EN_AP_A {
    #[doc = "0: Disable the interrupt for the power domain"]
    ENABLE = 0,
    #[doc = "1: Enable the interrupt for the power domain"]
    DISABLE = 1,
}
impl From<M4_SRAM_INTR_EN_AP_A> for bool {
    #[inline(always)]
    fn from(variant: M4_SRAM_INTR_EN_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4_SRAM_INTR_EN_AP` reader - M4 SRAM (access during low power) interrupt enable for AP"]
pub struct M4_SRAM_INTR_EN_AP_R(crate::FieldReader<bool, M4_SRAM_INTR_EN_AP_A>);
impl M4_SRAM_INTR_EN_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_INTR_EN_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4_SRAM_INTR_EN_AP_A {
        match self.bits {
            false => M4_SRAM_INTR_EN_AP_A::ENABLE,
            true => M4_SRAM_INTR_EN_AP_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == M4_SRAM_INTR_EN_AP_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == M4_SRAM_INTR_EN_AP_A::DISABLE
    }
}
impl core::ops::Deref for M4_SRAM_INTR_EN_AP_R {
    type Target = crate::FieldReader<bool, M4_SRAM_INTR_EN_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_INTR_EN_AP` writer - M4 SRAM (access during low power) interrupt enable for AP"]
pub struct M4_SRAM_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4_SRAM_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(M4_SRAM_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(M4_SRAM_INTR_EN_AP_A::DISABLE)
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
#[doc = "UART interrupt enable for AP"]
pub type UART_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `UART_INTR_EN_AP` reader - UART interrupt enable for AP"]
pub type UART_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `UART_INTR_EN_AP` writer - UART interrupt enable for AP"]
pub struct UART_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART_INTR_EN_AP_A::DISABLE)
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
#[doc = "Timer interrupt enable for AP"]
pub type TIMER_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `TIMER_INTR_EN_AP` reader - Timer interrupt enable for AP"]
pub type TIMER_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `TIMER_INTR_EN_AP` writer - Timer interrupt enable for AP"]
pub struct TIMER_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER_INTR_EN_AP_A::DISABLE)
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
#[doc = "WDT M4 interrupt enable for AP"]
pub type WDOG_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `WDOG_INTR_EN_AP` reader - WDT M4 interrupt enable for AP"]
pub type WDOG_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `WDOG_INTR_EN_AP` writer - WDT M4 interrupt enable for AP"]
pub struct WDOG_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDOG_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDOG_INTR_EN_AP_A::DISABLE)
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
#[doc = "WDT M4 Reset interrupt enable for AP"]
pub type WDOG_RST_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `WDOG_RST_EN_AP` reader - WDT M4 Reset interrupt enable for AP"]
pub type WDOG_RST_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `WDOG_RST_EN_AP` writer - WDT M4 Reset interrupt enable for AP"]
pub struct WDOG_RST_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_RST_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_RST_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDOG_RST_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDOG_RST_EN_AP_A::DISABLE)
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
#[doc = "Bus Timeout interrupt enable for AP"]
pub type TIMEOUT_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `TIMEOUT_INTR_EN_AP` reader - Bus Timeout interrupt enable for AP"]
pub type TIMEOUT_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `TIMEOUT_INTR_EN_AP` writer - Bus Timeout interrupt enable for AP"]
pub struct TIMEOUT_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMEOUT_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMEOUT_INTR_EN_AP_A::DISABLE)
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
#[doc = "M4 FPU interrupt enable for AP"]
pub type FPU_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `FPU_INTR_EN_AP` reader - M4 FPU interrupt enable for AP"]
pub type FPU_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `FPU_INTR_EN_AP` writer - M4 FPU interrupt enable for AP"]
pub struct FPU_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPU_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FPU_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FPU_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Packet FIFO Bank interrupt enable for AP"]
pub type PKFB_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `PKFB_INTR_EN_AP` reader - Packet FIFO Bank interrupt enable for AP"]
pub type PKFB_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `PKFB_INTR_EN_AP` writer - Packet FIFO Bank interrupt enable for AP"]
pub struct PKFB_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> PKFB_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PKFB_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PKFB_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PKFB_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "SPI Master interrupt enable for AP"]
pub type SPI_MS_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `SPI_MS_INTR_EN_AP` reader - SPI Master interrupt enable for AP"]
pub type SPI_MS_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `SPI_MS_INTR_EN_AP` writer - SPI Master interrupt enable for AP"]
pub struct SPI_MS_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MS_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_MS_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI_MS_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI_MS_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Config DMA interrupt enable for AP"]
pub type CFG_DMA_DONE_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `CFG_DMA_DONE_INTR_EN_AP` reader - Config DMA interrupt enable for AP"]
pub type CFG_DMA_DONE_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `CFG_DMA_DONE_INTR_EN_AP` writer - Config DMA interrupt enable for AP"]
pub struct CFG_DMA_DONE_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_DMA_DONE_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_DMA_DONE_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CFG_DMA_DONE_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CFG_DMA_DONE_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "PMU Timer interrupt enable for AP"]
pub type PMU_TMR_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `PMU_TMR_INTR_EN_AP` reader - PMU Timer interrupt enable for AP"]
pub type PMU_TMR_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `PMU_TMR_INTR_EN_AP` writer - PMU Timer interrupt enable for AP"]
pub struct PMU_TMR_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_TMR_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMU_TMR_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PMU_TMR_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PMU_TMR_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "ADC interrupt enable for AP"]
pub type ADC_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `ADC_INTR_EN_AP` reader - ADC interrupt enable for AP"]
pub type ADC_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `ADC_INTR_EN_AP` writer - ADC interrupt enable for AP"]
pub struct ADC_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "RTC interrupt enable for AP"]
pub type RTC_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `RTC_INTR_EN_AP` reader - RTC interrupt enable for AP"]
pub type RTC_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `RTC_INTR_EN_AP` writer - RTC interrupt enable for AP"]
pub struct RTC_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Reset interrupt enable for AP"]
pub type RST_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `RST_INTR_EN_AP` reader - Reset interrupt enable for AP"]
pub type RST_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `RST_INTR_EN_AP` writer - Reset interrupt enable for AP"]
pub struct RST_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RST_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RST_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "FFE0 Other interrupt enable for AP"]
pub type FFE0_INTR_OTHERS_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `FFE0_INTR_OTHERS_EN_AP` reader - FFE0 Other interrupt enable for AP"]
pub type FFE0_INTR_OTHERS_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `FFE0_INTR_OTHERS_EN_AP` writer - FFE0 Other interrupt enable for AP"]
pub struct FFE0_INTR_OTHERS_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_INTR_OTHERS_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFE0_INTR_OTHERS_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FFE0_INTR_OTHERS_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FFE0_INTR_OTHERS_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "AP Boot interrupt enable for AP"]
pub type APBOOT_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `APBOOT_EN_AP` reader - AP Boot interrupt enable for AP"]
pub type APBOOT_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `APBOOT_EN_AP` writer - AP Boot interrupt enable for AP"]
pub struct APBOOT_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> APBOOT_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APBOOT_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(APBOOT_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(APBOOT_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "LDO30 absence of power good interrupt enable for AP"]
pub type LDO30_PG_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `LDO30_PG_INTR_EN_AP` reader - LDO30 absence of power good interrupt enable for AP"]
pub type LDO30_PG_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `LDO30_PG_INTR_EN_AP` writer - LDO30 absence of power good interrupt enable for AP"]
pub struct LDO30_PG_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO30_PG_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDO30_PG_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LDO30_PG_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LDO30_PG_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "LDO50 absence of power good interrupt enable for AP"]
pub type LDO50_PG_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `LDO50_PG_INTR_EN_AP` reader - LDO50 absence of power good interrupt enable for AP"]
pub type LDO50_PG_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `LDO50_PG_INTR_EN_AP` writer - LDO50 absence of power good interrupt enable for AP"]
pub struct LDO50_PG_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO50_PG_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDO50_PG_INTR_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LDO50_PG_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LDO50_PG_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "SRAM_128KB_TIMEOUT interrupt enable for AP"]
pub type SRAM_128KB_TIMEOUT_INTR_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `SRAM_128KB_TIMEOUT_INTR_EN_AP` reader - SRAM_128KB_TIMEOUT interrupt enable for AP"]
pub type SRAM_128KB_TIMEOUT_INTR_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `SRAM_128KB_TIMEOUT_INTR_EN_AP` writer - SRAM_128KB_TIMEOUT interrupt enable for AP"]
pub struct SRAM_128KB_TIMEOUT_INTR_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_128KB_TIMEOUT_INTR_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(
        self,
        variant: SRAM_128KB_TIMEOUT_INTR_EN_AP_A,
    ) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_128KB_TIMEOUT_INTR_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_128KB_TIMEOUT_INTR_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "LPSD Voice detected interrupt enable for AP"]
pub type LPSD_VOICE_DET_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `LPSD_VOICE_DET_EN_AP` reader - LPSD Voice detected interrupt enable for AP"]
pub type LPSD_VOICE_DET_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `LPSD_VOICE_DET_EN_AP` writer - LPSD Voice detected interrupt enable for AP"]
pub struct LPSD_VOICE_DET_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_VOICE_DET_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSD_VOICE_DET_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPSD_VOICE_DET_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPSD_VOICE_DET_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Digital Mic interrupt enable for AP"]
pub type DMIC_VOICE_DET_EN_AP_A = M4_SRAM_INTR_EN_AP_A;
#[doc = "Field `DMIC_VOICE_DET_EN_AP` reader - Digital Mic interrupt enable for AP"]
pub type DMIC_VOICE_DET_EN_AP_R = M4_SRAM_INTR_EN_AP_R;
#[doc = "Field `DMIC_VOICE_DET_EN_AP` writer - Digital Mic interrupt enable for AP"]
pub struct DMIC_VOICE_DET_EN_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_VOICE_DET_EN_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIC_VOICE_DET_EN_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the interrupt for the power domain"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMIC_VOICE_DET_EN_AP_A::ENABLE)
    }
    #[doc = "Enable the interrupt for the power domain"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMIC_VOICE_DET_EN_AP_A::DISABLE)
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
            (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - M4 SRAM (access during low power) interrupt enable for AP"]
    #[inline(always)]
    pub fn m4_sram_intr_en_ap(&self) -> M4_SRAM_INTR_EN_AP_R {
        M4_SRAM_INTR_EN_AP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART interrupt enable for AP"]
    #[inline(always)]
    pub fn uart_intr_en_ap(&self) -> UART_INTR_EN_AP_R {
        UART_INTR_EN_AP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer interrupt enable for AP"]
    #[inline(always)]
    pub fn timer_intr_en_ap(&self) -> TIMER_INTR_EN_AP_R {
        TIMER_INTR_EN_AP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDT M4 interrupt enable for AP"]
    #[inline(always)]
    pub fn wdog_intr_en_ap(&self) -> WDOG_INTR_EN_AP_R {
        WDOG_INTR_EN_AP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WDT M4 Reset interrupt enable for AP"]
    #[inline(always)]
    pub fn wdog_rst_en_ap(&self) -> WDOG_RST_EN_AP_R {
        WDOG_RST_EN_AP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus Timeout interrupt enable for AP"]
    #[inline(always)]
    pub fn timeout_intr_en_ap(&self) -> TIMEOUT_INTR_EN_AP_R {
        TIMEOUT_INTR_EN_AP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - M4 FPU interrupt enable for AP"]
    #[inline(always)]
    pub fn fpu_intr_en_ap(&self) -> FPU_INTR_EN_AP_R {
        FPU_INTR_EN_AP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Packet FIFO Bank interrupt enable for AP"]
    #[inline(always)]
    pub fn pkfb_intr_en_ap(&self) -> PKFB_INTR_EN_AP_R {
        PKFB_INTR_EN_AP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI Master interrupt enable for AP"]
    #[inline(always)]
    pub fn spi_ms_intr_en_ap(&self) -> SPI_MS_INTR_EN_AP_R {
        SPI_MS_INTR_EN_AP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Config DMA interrupt enable for AP"]
    #[inline(always)]
    pub fn cfg_dma_done_intr_en_ap(&self) -> CFG_DMA_DONE_INTR_EN_AP_R {
        CFG_DMA_DONE_INTR_EN_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PMU Timer interrupt enable for AP"]
    #[inline(always)]
    pub fn pmu_tmr_intr_en_ap(&self) -> PMU_TMR_INTR_EN_AP_R {
        PMU_TMR_INTR_EN_AP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC interrupt enable for AP"]
    #[inline(always)]
    pub fn adc_intr_en_ap(&self) -> ADC_INTR_EN_AP_R {
        ADC_INTR_EN_AP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC interrupt enable for AP"]
    #[inline(always)]
    pub fn rtc_intr_en_ap(&self) -> RTC_INTR_EN_AP_R {
        RTC_INTR_EN_AP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reset interrupt enable for AP"]
    #[inline(always)]
    pub fn rst_intr_en_ap(&self) -> RST_INTR_EN_AP_R {
        RST_INTR_EN_AP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FFE0 Other interrupt enable for AP"]
    #[inline(always)]
    pub fn ffe0_intr_others_en_ap(&self) -> FFE0_INTR_OTHERS_EN_AP_R {
        FFE0_INTR_OTHERS_EN_AP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AP Boot interrupt enable for AP"]
    #[inline(always)]
    pub fn apboot_en_ap(&self) -> APBOOT_EN_AP_R {
        APBOOT_EN_AP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LDO30 absence of power good interrupt enable for AP"]
    #[inline(always)]
    pub fn ldo30_pg_intr_en_ap(&self) -> LDO30_PG_INTR_EN_AP_R {
        LDO30_PG_INTR_EN_AP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LDO50 absence of power good interrupt enable for AP"]
    #[inline(always)]
    pub fn ldo50_pg_intr_en_ap(&self) -> LDO50_PG_INTR_EN_AP_R {
        LDO50_PG_INTR_EN_AP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SRAM_128KB_TIMEOUT interrupt enable for AP"]
    #[inline(always)]
    pub fn sram_128kb_timeout_intr_en_ap(
        &self,
    ) -> SRAM_128KB_TIMEOUT_INTR_EN_AP_R {
        SRAM_128KB_TIMEOUT_INTR_EN_AP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LPSD Voice detected interrupt enable for AP"]
    #[inline(always)]
    pub fn lpsd_voice_det_en_ap(&self) -> LPSD_VOICE_DET_EN_AP_R {
        LPSD_VOICE_DET_EN_AP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Digital Mic interrupt enable for AP"]
    #[inline(always)]
    pub fn dmic_voice_det_en_ap(&self) -> DMIC_VOICE_DET_EN_AP_R {
        DMIC_VOICE_DET_EN_AP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - M4 SRAM (access during low power) interrupt enable for AP"]
    #[inline(always)]
    pub fn m4_sram_intr_en_ap(&mut self) -> M4_SRAM_INTR_EN_AP_W {
        M4_SRAM_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 1 - UART interrupt enable for AP"]
    #[inline(always)]
    pub fn uart_intr_en_ap(&mut self) -> UART_INTR_EN_AP_W {
        UART_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 2 - Timer interrupt enable for AP"]
    #[inline(always)]
    pub fn timer_intr_en_ap(&mut self) -> TIMER_INTR_EN_AP_W {
        TIMER_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 3 - WDT M4 interrupt enable for AP"]
    #[inline(always)]
    pub fn wdog_intr_en_ap(&mut self) -> WDOG_INTR_EN_AP_W {
        WDOG_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 4 - WDT M4 Reset interrupt enable for AP"]
    #[inline(always)]
    pub fn wdog_rst_en_ap(&mut self) -> WDOG_RST_EN_AP_W {
        WDOG_RST_EN_AP_W { w: self }
    }
    #[doc = "Bit 5 - Bus Timeout interrupt enable for AP"]
    #[inline(always)]
    pub fn timeout_intr_en_ap(&mut self) -> TIMEOUT_INTR_EN_AP_W {
        TIMEOUT_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 6 - M4 FPU interrupt enable for AP"]
    #[inline(always)]
    pub fn fpu_intr_en_ap(&mut self) -> FPU_INTR_EN_AP_W {
        FPU_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 7 - Packet FIFO Bank interrupt enable for AP"]
    #[inline(always)]
    pub fn pkfb_intr_en_ap(&mut self) -> PKFB_INTR_EN_AP_W {
        PKFB_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 10 - SPI Master interrupt enable for AP"]
    #[inline(always)]
    pub fn spi_ms_intr_en_ap(&mut self) -> SPI_MS_INTR_EN_AP_W {
        SPI_MS_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 11 - Config DMA interrupt enable for AP"]
    #[inline(always)]
    pub fn cfg_dma_done_intr_en_ap(&mut self) -> CFG_DMA_DONE_INTR_EN_AP_W {
        CFG_DMA_DONE_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 12 - PMU Timer interrupt enable for AP"]
    #[inline(always)]
    pub fn pmu_tmr_intr_en_ap(&mut self) -> PMU_TMR_INTR_EN_AP_W {
        PMU_TMR_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 13 - ADC interrupt enable for AP"]
    #[inline(always)]
    pub fn adc_intr_en_ap(&mut self) -> ADC_INTR_EN_AP_W {
        ADC_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 14 - RTC interrupt enable for AP"]
    #[inline(always)]
    pub fn rtc_intr_en_ap(&mut self) -> RTC_INTR_EN_AP_W {
        RTC_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 15 - Reset interrupt enable for AP"]
    #[inline(always)]
    pub fn rst_intr_en_ap(&mut self) -> RST_INTR_EN_AP_W {
        RST_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 16 - FFE0 Other interrupt enable for AP"]
    #[inline(always)]
    pub fn ffe0_intr_others_en_ap(&mut self) -> FFE0_INTR_OTHERS_EN_AP_W {
        FFE0_INTR_OTHERS_EN_AP_W { w: self }
    }
    #[doc = "Bit 18 - AP Boot interrupt enable for AP"]
    #[inline(always)]
    pub fn apboot_en_ap(&mut self) -> APBOOT_EN_AP_W {
        APBOOT_EN_AP_W { w: self }
    }
    #[doc = "Bit 19 - LDO30 absence of power good interrupt enable for AP"]
    #[inline(always)]
    pub fn ldo30_pg_intr_en_ap(&mut self) -> LDO30_PG_INTR_EN_AP_W {
        LDO30_PG_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 20 - LDO50 absence of power good interrupt enable for AP"]
    #[inline(always)]
    pub fn ldo50_pg_intr_en_ap(&mut self) -> LDO50_PG_INTR_EN_AP_W {
        LDO50_PG_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 21 - SRAM_128KB_TIMEOUT interrupt enable for AP"]
    #[inline(always)]
    pub fn sram_128kb_timeout_intr_en_ap(
        &mut self,
    ) -> SRAM_128KB_TIMEOUT_INTR_EN_AP_W {
        SRAM_128KB_TIMEOUT_INTR_EN_AP_W { w: self }
    }
    #[doc = "Bit 22 - LPSD Voice detected interrupt enable for AP"]
    #[inline(always)]
    pub fn lpsd_voice_det_en_ap(&mut self) -> LPSD_VOICE_DET_EN_AP_W {
        LPSD_VOICE_DET_EN_AP_W { w: self }
    }
    #[doc = "Bit 23 - Digital Mic interrupt enable for AP"]
    #[inline(always)]
    pub fn dmic_voice_det_en_ap(&mut self) -> DMIC_VOICE_DET_EN_AP_W {
        DMIC_VOICE_DET_EN_AP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various interrupt enable for AP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [other_intr_en_ap](index.html) module"]
pub struct OTHER_INTR_EN_AP_SPEC;
impl crate::RegisterSpec for OTHER_INTR_EN_AP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [other_intr_en_ap::R](R) reader structure"]
impl crate::Readable for OTHER_INTR_EN_AP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [other_intr_en_ap::W](W) writer structure"]
impl crate::Writable for OTHER_INTR_EN_AP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTHER_INTR_EN_AP to value 0"]
impl crate::Resettable for OTHER_INTR_EN_AP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
