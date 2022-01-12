#[doc = "Register `CFG_FLASH_HEADER` reader"]
pub struct R(crate::R<CFG_FLASH_HEADER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_FLASH_HEADER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_FLASH_HEADER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_FLASH_HEADER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dma_boot_xfr_size` reader - number of double words (8 bytes) for the SPI to transfer (minus 1)"]
pub struct DMA_BOOT_XFR_SIZE_R(crate::FieldReader<u16, u16>);
impl DMA_BOOT_XFR_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DMA_BOOT_XFR_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_BOOT_XFR_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_spi_clik_divide` reader - SPI data clock out divides the ssi_clk (value in bytes*2)"]
pub struct DMA_SPI_CLIK_DIVIDE_R(crate::FieldReader<u8, u8>);
impl DMA_SPI_CLIK_DIVIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_SPI_CLIK_DIVIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SPI_CLIK_DIVIDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_device_id` reader - Device ID"]
pub struct DMA_DEVICE_ID_R(crate::FieldReader<u8, u8>);
impl DMA_DEVICE_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEVICE_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEVICE_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - number of double words (8 bytes) for the SPI to transfer (minus 1)"]
    #[inline(always)]
    pub fn dma_boot_xfr_size(&self) -> DMA_BOOT_XFR_SIZE_R {
        DMA_BOOT_XFR_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - SPI data clock out divides the ssi_clk (value in bytes*2)"]
    #[inline(always)]
    pub fn dma_spi_clik_divide(&self) -> DMA_SPI_CLIK_DIVIDE_R {
        DMA_SPI_CLIK_DIVIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Device ID"]
    #[inline(always)]
    pub fn dma_device_id(&self) -> DMA_DEVICE_ID_R {
        DMA_DEVICE_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Header values read from EEPROM : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_flash_header](index.html) module"]
pub struct CFG_FLASH_HEADER_SPEC;
impl crate::RegisterSpec for CFG_FLASH_HEADER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_flash_header::R](R) reader structure"]
impl crate::Readable for CFG_FLASH_HEADER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFG_FLASH_HEADER to value 0"]
impl crate::Resettable for CFG_FLASH_HEADER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
