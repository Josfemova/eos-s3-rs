#[doc = "Register `OTHER_INTR` reader"]
pub struct R(crate::R<OTHER_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTHER_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTHER_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTHER_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTHER_INTR` writer"]
pub struct W(crate::W<OTHER_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTHER_INTR_SPEC>;
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
impl From<crate::W<OTHER_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTHER_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M4_SRAM_INTR` reader - Interrupt detected for M4 SRAM (access during low power)"]
pub struct M4_SRAM_INTR_R(crate::FieldReader<bool, bool>);
impl M4_SRAM_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_SRAM_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4_SRAM_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_SRAM_INTR` writer - Interrupt detected for M4 SRAM (access during low power)"]
pub struct M4_SRAM_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_SRAM_INTR_W<'a> {
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
#[doc = "Field `UART_INTR` reader - Interrupt detected for UART"]
pub struct UART_INTR_R(crate::FieldReader<bool, bool>);
impl UART_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_INTR` writer - Interrupt detected for UART"]
pub struct UART_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_INTR_W<'a> {
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
#[doc = "Field `TIMER_INTR` reader - Interrupt detected for timer"]
pub struct TIMER_INTR_R(crate::FieldReader<bool, bool>);
impl TIMER_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_INTR` writer - Interrupt detected for timer"]
pub struct TIMER_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_INTR_W<'a> {
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
#[doc = "Field `WDOG_INTR` reader - Interrupt detected for WDT M4"]
pub struct WDOG_INTR_R(crate::FieldReader<bool, bool>);
impl WDOG_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_INTR` writer - Interrupt detected for WDT M4"]
pub struct WDOG_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_INTR_W<'a> {
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
#[doc = "Field `WDOG_RST` reader - Interrupt detected for WDT M4 Reset"]
pub struct WDOG_RST_R(crate::FieldReader<bool, bool>);
impl WDOG_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_RST` writer - Interrupt detected for WDT M4 Reset"]
pub struct WDOG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_RST_W<'a> {
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
#[doc = "Field `TIMEOUT_INTR` reader - Interrupt detected for bus timeout"]
pub struct TIMEOUT_INTR_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT_INTR` writer - Interrupt detected for bus timeout"]
pub struct TIMEOUT_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_INTR_W<'a> {
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
#[doc = "Field `FPU_INTR` reader - Interrupt detected for M4 FPU"]
pub struct FPU_INTR_R(crate::FieldReader<bool, bool>);
impl FPU_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPU_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPU_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPU_INTR` writer - Interrupt detected for M4 FPU"]
pub struct FPU_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPU_INTR_W<'a> {
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
#[doc = "Field `PKFB_INTR` reader - Interrupt detected for Packet FIFO Bank"]
pub struct PKFB_INTR_R(crate::FieldReader<bool, bool>);
impl PKFB_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PKFB_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKFB_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKFB_INTR` writer - Interrupt detected for Packet FIFO Bank"]
pub struct PKFB_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PKFB_INTR_W<'a> {
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
#[doc = "Field `SPI_MS_INTR` reader - Interrupt detected for SPI Master"]
pub struct SPI_MS_INTR_R(crate::FieldReader<bool, bool>);
impl SPI_MS_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_MS_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MS_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_MS_INTR` writer - Interrupt detected for SPI Master"]
pub struct SPI_MS_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MS_INTR_W<'a> {
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
#[doc = "Field `CFG_DMA_INTR` reader - Interrupt detected for Config DMA"]
pub struct CFG_DMA_INTR_R(crate::FieldReader<bool, bool>);
impl CFG_DMA_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_DMA_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_DMA_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_DMA_INTR` writer - Interrupt detected for Config DMA"]
pub struct CFG_DMA_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_DMA_INTR_W<'a> {
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
#[doc = "Field `PMU_TMR_INTR` reader - Interrupt detected for PMU Timer"]
pub struct PMU_TMR_INTR_R(crate::FieldReader<bool, bool>);
impl PMU_TMR_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMU_TMR_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_TMR_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_TMR_INTR` writer - Interrupt detected for PMU Timer"]
pub struct PMU_TMR_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_TMR_INTR_W<'a> {
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
#[doc = "Field `ADC_INTR` reader - Interrupt detected for ADC"]
pub struct ADC_INTR_R(crate::FieldReader<bool, bool>);
impl ADC_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_INTR` writer - Interrupt detected for ADC"]
pub struct ADC_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INTR_W<'a> {
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
#[doc = "Field `RTC_INTR` reader - Interrupt detected for RTC"]
pub struct RTC_INTR_R(crate::FieldReader<bool, bool>);
impl RTC_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_INTR` writer - Interrupt detected for RTC"]
pub struct RTC_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_INTR_W<'a> {
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
#[doc = "Field `RST_INTR` reader - Interrupt detected for Reset"]
pub struct RST_INTR_R(crate::FieldReader<bool, bool>);
impl RST_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_INTR` writer - Interrupt detected for Reset"]
pub struct RST_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_INTR_W<'a> {
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
#[doc = "Field `FFE0_INTR_OTHERS` reader - Interrupt detected for FFE0 other interrupts"]
pub struct FFE0_INTR_OTHERS_R(crate::FieldReader<bool, bool>);
impl FFE0_INTR_OTHERS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_INTR_OTHERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_INTR_OTHERS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_INTR_OTHERS` writer - Interrupt detected for FFE0 other interrupts"]
pub struct FFE0_INTR_OTHERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_INTR_OTHERS_W<'a> {
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
#[doc = "Field `APBOOT_EN_INTR` reader - Interrupt detected for AP boot"]
pub struct APBOOT_EN_INTR_R(crate::FieldReader<bool, bool>);
impl APBOOT_EN_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APBOOT_EN_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APBOOT_EN_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APBOOT_EN_INTR` writer - Interrupt detected for AP boot"]
pub struct APBOOT_EN_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> APBOOT_EN_INTR_W<'a> {
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
#[doc = "Field `LDO30_PG_INTR` reader - Interrupt detected for absence of LDO30 power good"]
pub struct LDO30_PG_INTR_R(crate::FieldReader<bool, bool>);
impl LDO30_PG_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO30_PG_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO30_PG_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO30_PG_INTR` writer - Interrupt detected for absence of LDO30 power good"]
pub struct LDO30_PG_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO30_PG_INTR_W<'a> {
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
#[doc = "Field `LDO50_PG_INTR` reader - Interrupt detected for absence of LDO30 power good"]
pub struct LDO50_PG_INTR_R(crate::FieldReader<bool, bool>);
impl LDO50_PG_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO50_PG_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO50_PG_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO50_PG_INTR` writer - Interrupt detected for absence of LDO30 power good"]
pub struct LDO50_PG_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO50_PG_INTR_W<'a> {
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
#[doc = "Field `LPSD_VOICE_DET` reader - Interrupt detected for LPSD Voice"]
pub struct LPSD_VOICE_DET_R(crate::FieldReader<bool, bool>);
impl LPSD_VOICE_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSD_VOICE_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPSD_VOICE_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSD_VOICE_DET` writer - Interrupt detected for LPSD Voice"]
pub struct LPSD_VOICE_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSD_VOICE_DET_W<'a> {
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
#[doc = "Field `DMIC_VOICE_DET` reader - Interrupt detected for Digital MIC"]
pub struct DMIC_VOICE_DET_R(crate::FieldReader<bool, bool>);
impl DMIC_VOICE_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMIC_VOICE_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMIC_VOICE_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMIC_VOICE_DET` writer - Interrupt detected for Digital MIC"]
pub struct DMIC_VOICE_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_VOICE_DET_W<'a> {
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
    #[doc = "Bit 0 - Interrupt detected for M4 SRAM (access during low power)"]
    #[inline(always)]
    pub fn m4_sram_intr(&self) -> M4_SRAM_INTR_R {
        M4_SRAM_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt detected for UART"]
    #[inline(always)]
    pub fn uart_intr(&self) -> UART_INTR_R {
        UART_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt detected for timer"]
    #[inline(always)]
    pub fn timer_intr(&self) -> TIMER_INTR_R {
        TIMER_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt detected for WDT M4"]
    #[inline(always)]
    pub fn wdog_intr(&self) -> WDOG_INTR_R {
        WDOG_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt detected for WDT M4 Reset"]
    #[inline(always)]
    pub fn wdog_rst(&self) -> WDOG_RST_R {
        WDOG_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt detected for bus timeout"]
    #[inline(always)]
    pub fn timeout_intr(&self) -> TIMEOUT_INTR_R {
        TIMEOUT_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt detected for M4 FPU"]
    #[inline(always)]
    pub fn fpu_intr(&self) -> FPU_INTR_R {
        FPU_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt detected for Packet FIFO Bank"]
    #[inline(always)]
    pub fn pkfb_intr(&self) -> PKFB_INTR_R {
        PKFB_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt detected for SPI Master"]
    #[inline(always)]
    pub fn spi_ms_intr(&self) -> SPI_MS_INTR_R {
        SPI_MS_INTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt detected for Config DMA"]
    #[inline(always)]
    pub fn cfg_dma_intr(&self) -> CFG_DMA_INTR_R {
        CFG_DMA_INTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt detected for PMU Timer"]
    #[inline(always)]
    pub fn pmu_tmr_intr(&self) -> PMU_TMR_INTR_R {
        PMU_TMR_INTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt detected for ADC"]
    #[inline(always)]
    pub fn adc_intr(&self) -> ADC_INTR_R {
        ADC_INTR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt detected for RTC"]
    #[inline(always)]
    pub fn rtc_intr(&self) -> RTC_INTR_R {
        RTC_INTR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt detected for Reset"]
    #[inline(always)]
    pub fn rst_intr(&self) -> RST_INTR_R {
        RST_INTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt detected for FFE0 other interrupts"]
    #[inline(always)]
    pub fn ffe0_intr_others(&self) -> FFE0_INTR_OTHERS_R {
        FFE0_INTR_OTHERS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt detected for AP boot"]
    #[inline(always)]
    pub fn apboot_en_intr(&self) -> APBOOT_EN_INTR_R {
        APBOOT_EN_INTR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt detected for absence of LDO30 power good"]
    #[inline(always)]
    pub fn ldo30_pg_intr(&self) -> LDO30_PG_INTR_R {
        LDO30_PG_INTR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt detected for absence of LDO30 power good"]
    #[inline(always)]
    pub fn ldo50_pg_intr(&self) -> LDO50_PG_INTR_R {
        LDO50_PG_INTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt detected for LPSD Voice"]
    #[inline(always)]
    pub fn lpsd_voice_det(&self) -> LPSD_VOICE_DET_R {
        LPSD_VOICE_DET_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt detected for Digital MIC"]
    #[inline(always)]
    pub fn dmic_voice_det(&self) -> DMIC_VOICE_DET_R {
        DMIC_VOICE_DET_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt detected for M4 SRAM (access during low power)"]
    #[inline(always)]
    pub fn m4_sram_intr(&mut self) -> M4_SRAM_INTR_W {
        M4_SRAM_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt detected for UART"]
    #[inline(always)]
    pub fn uart_intr(&mut self) -> UART_INTR_W {
        UART_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt detected for timer"]
    #[inline(always)]
    pub fn timer_intr(&mut self) -> TIMER_INTR_W {
        TIMER_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt detected for WDT M4"]
    #[inline(always)]
    pub fn wdog_intr(&mut self) -> WDOG_INTR_W {
        WDOG_INTR_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt detected for WDT M4 Reset"]
    #[inline(always)]
    pub fn wdog_rst(&mut self) -> WDOG_RST_W {
        WDOG_RST_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt detected for bus timeout"]
    #[inline(always)]
    pub fn timeout_intr(&mut self) -> TIMEOUT_INTR_W {
        TIMEOUT_INTR_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt detected for M4 FPU"]
    #[inline(always)]
    pub fn fpu_intr(&mut self) -> FPU_INTR_W {
        FPU_INTR_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt detected for Packet FIFO Bank"]
    #[inline(always)]
    pub fn pkfb_intr(&mut self) -> PKFB_INTR_W {
        PKFB_INTR_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt detected for SPI Master"]
    #[inline(always)]
    pub fn spi_ms_intr(&mut self) -> SPI_MS_INTR_W {
        SPI_MS_INTR_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt detected for Config DMA"]
    #[inline(always)]
    pub fn cfg_dma_intr(&mut self) -> CFG_DMA_INTR_W {
        CFG_DMA_INTR_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt detected for PMU Timer"]
    #[inline(always)]
    pub fn pmu_tmr_intr(&mut self) -> PMU_TMR_INTR_W {
        PMU_TMR_INTR_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt detected for ADC"]
    #[inline(always)]
    pub fn adc_intr(&mut self) -> ADC_INTR_W {
        ADC_INTR_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt detected for RTC"]
    #[inline(always)]
    pub fn rtc_intr(&mut self) -> RTC_INTR_W {
        RTC_INTR_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt detected for Reset"]
    #[inline(always)]
    pub fn rst_intr(&mut self) -> RST_INTR_W {
        RST_INTR_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt detected for FFE0 other interrupts"]
    #[inline(always)]
    pub fn ffe0_intr_others(&mut self) -> FFE0_INTR_OTHERS_W {
        FFE0_INTR_OTHERS_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt detected for AP boot"]
    #[inline(always)]
    pub fn apboot_en_intr(&mut self) -> APBOOT_EN_INTR_W {
        APBOOT_EN_INTR_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt detected for absence of LDO30 power good"]
    #[inline(always)]
    pub fn ldo30_pg_intr(&mut self) -> LDO30_PG_INTR_W {
        LDO30_PG_INTR_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt detected for absence of LDO30 power good"]
    #[inline(always)]
    pub fn ldo50_pg_intr(&mut self) -> LDO50_PG_INTR_W {
        LDO50_PG_INTR_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt detected for LPSD Voice"]
    #[inline(always)]
    pub fn lpsd_voice_det(&mut self) -> LPSD_VOICE_DET_W {
        LPSD_VOICE_DET_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt detected for Digital MIC"]
    #[inline(always)]
    pub fn dmic_voice_det(&mut self) -> DMIC_VOICE_DET_W {
        DMIC_VOICE_DET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of interrupt triggers detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [other_intr](index.html) module"]
pub struct OTHER_INTR_SPEC;
impl crate::RegisterSpec for OTHER_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [other_intr::R](R) reader structure"]
impl crate::Readable for OTHER_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [other_intr::W](W) writer structure"]
impl crate::Writable for OTHER_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTHER_INTR to value 0"]
impl crate::Resettable for OTHER_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
