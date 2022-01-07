#[doc = "Register `PWR_DWN_SCH` reader"]
pub struct R(crate::R<PWR_DWN_SCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_DWN_SCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_DWN_SCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_DWN_SCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_DWN_SCH` writer"]
pub struct W(crate::W<PWR_DWN_SCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_DWN_SCH_SPEC>;
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
impl From<crate::W<PWR_DWN_SCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_DWN_SCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FFE and FB simultaneous wakeup-event behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFEFB_WU_A {
    #[doc = "0: If FFE and FB WU event happen at same time, FB will wake up first, then FFE."]
    SECUENTIAL = 0,
    #[doc = "1: If FFE and FB WU event happen at same time, FFE and FB will wake up in parallel"]
    PARALLEL = 1,
}
impl From<FFEFB_WU_A> for bool {
    #[inline(always)]
    fn from(variant: FFEFB_WU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFEFB_WU` reader - FFE and FB simultaneous wakeup-event behaviour"]
pub struct FFEFB_WU_R(crate::FieldReader<bool, FFEFB_WU_A>);
impl FFEFB_WU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFEFB_WU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFEFB_WU_A {
        match self.bits {
            false => FFEFB_WU_A::SECUENTIAL,
            true => FFEFB_WU_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == FFEFB_WU_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == FFEFB_WU_A::PARALLEL
    }
}
impl core::ops::Deref for FFEFB_WU_R {
    type Target = crate::FieldReader<bool, FFEFB_WU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFEFB_WU` writer - FFE and FB simultaneous wakeup-event behaviour"]
pub struct FFEFB_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> FFEFB_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFEFB_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If FFE and FB WU event happen at same time, FB will wake up first, then FFE."]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(FFEFB_WU_A::SECUENTIAL)
    }
    #[doc = "If FFE and FB WU event happen at same time, FFE and FB will wake up in parallel"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(FFEFB_WU_A::PARALLEL)
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
#[doc = "SRAM simultaneous wakeup-event behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_WU_A {
    #[doc = "0: If more than one SRAM (M4S1 ~ M4S15) WU event happen at same time, these SRAMs will wake up in the following priority. M4S1->M4S2->….->M4S14-> M4S15 (M4S1 has the highest priority and M4S15 has the lowest priority)"]
    SECUENTIAL = 0,
    #[doc = "1: If more than one SRAM(M4S1 ~ M4S15, NOT include M4S0) WU event happen at same time, these SRAMs will wake up in parallel."]
    PARALLEL = 1,
}
impl From<SRAM_WU_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_WU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_WU` reader - SRAM simultaneous wakeup-event behaviour"]
pub struct SRAM_WU_R(crate::FieldReader<bool, SRAM_WU_A>);
impl SRAM_WU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_WU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_WU_A {
        match self.bits {
            false => SRAM_WU_A::SECUENTIAL,
            true => SRAM_WU_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == SRAM_WU_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == SRAM_WU_A::PARALLEL
    }
}
impl core::ops::Deref for SRAM_WU_R {
    type Target = crate::FieldReader<bool, SRAM_WU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_WU` writer - SRAM simultaneous wakeup-event behaviour"]
pub struct SRAM_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If more than one SRAM (M4S1 ~ M4S15) WU event happen at same time, these SRAMs will wake up in the following priority. M4S1->M4S2->….->M4S14-> M4S15 (M4S1 has the highest priority and M4S15 has the lowest priority)"]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(SRAM_WU_A::SECUENTIAL)
    }
    #[doc = "If more than one SRAM(M4S1 ~ M4S15, NOT include M4S0) WU event happen at same time, these SRAMs will wake up in parallel."]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(SRAM_WU_A::PARALLEL)
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
#[doc = "Audio simultaneous wakeup-event behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_WU_A {
    #[doc = "0: If more than one AUDIO Blocks (AD0 ~ AD5) WU event happen at same time, these Blocks will wake up in the following priority. AD5->AD1->AD2->AD0->AD3->AD4. (AD5 has the highest priority and AD4 has the lowest priority)"]
    SECUENTIAL = 0,
    #[doc = "1: If more than one AUDIO Blocks (AD0 ~ AD5) WU event happen at same time, these AUDIO power domain will wake up in parallel"]
    PARALLEL = 1,
}
impl From<AUDIO_WU_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_WU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDIO_WU` reader - Audio simultaneous wakeup-event behaviour"]
pub struct AUDIO_WU_R(crate::FieldReader<bool, AUDIO_WU_A>);
impl AUDIO_WU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUDIO_WU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_WU_A {
        match self.bits {
            false => AUDIO_WU_A::SECUENTIAL,
            true => AUDIO_WU_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == AUDIO_WU_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == AUDIO_WU_A::PARALLEL
    }
}
impl core::ops::Deref for AUDIO_WU_R {
    type Target = crate::FieldReader<bool, AUDIO_WU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUDIO_WU` writer - Audio simultaneous wakeup-event behaviour"]
pub struct AUDIO_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If more than one AUDIO Blocks (AD0 ~ AD5) WU event happen at same time, these Blocks will wake up in the following priority. AD5->AD1->AD2->AD0->AD3->AD4. (AD5 has the highest priority and AD4 has the lowest priority)"]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(AUDIO_WU_A::SECUENTIAL)
    }
    #[doc = "If more than one AUDIO Blocks (AD0 ~ AD5) WU event happen at same time, these AUDIO power domain will wake up in parallel"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(AUDIO_WU_A::PARALLEL)
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
#[doc = "M4 and M4S0 simultaneous wakeup-event behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4M4S0_WU_A {
    #[doc = "0: Not defined. guess: First M4 then M4S0"]
    SECUENTIAL = 0,
    #[doc = "1: If M4 and M4S0 WU event happen at same time, then M4 and M4S0 will wake up at same time"]
    PARALLEL = 1,
}
impl From<M4M4S0_WU_A> for bool {
    #[inline(always)]
    fn from(variant: M4M4S0_WU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4M4S0_WU` reader - M4 and M4S0 simultaneous wakeup-event behaviour"]
pub struct M4M4S0_WU_R(crate::FieldReader<bool, M4M4S0_WU_A>);
impl M4M4S0_WU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4M4S0_WU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4M4S0_WU_A {
        match self.bits {
            false => M4M4S0_WU_A::SECUENTIAL,
            true => M4M4S0_WU_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == M4M4S0_WU_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == M4M4S0_WU_A::PARALLEL
    }
}
impl core::ops::Deref for M4M4S0_WU_R {
    type Target = crate::FieldReader<bool, M4M4S0_WU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4M4S0_WU` writer - M4 and M4S0 simultaneous wakeup-event behaviour"]
pub struct M4M4S0_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> M4M4S0_WU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4M4S0_WU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not defined. guess: First M4 then M4S0"]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(M4M4S0_WU_A::SECUENTIAL)
    }
    #[doc = "If M4 and M4S0 WU event happen at same time, then M4 and M4S0 will wake up at same time"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(M4M4S0_WU_A::PARALLEL)
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
#[doc = "FFE and FB simultaneous power-down behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFEFB_PD_A {
    #[doc = "0: If FFE and FB PD event happen at same time, FB will power down first, then FFE."]
    SECUENTIAL = 0,
    #[doc = "1: If FFE and FB PD event happen at same time, FFE and FB will power down in parallel"]
    PARALLEL = 1,
}
impl From<FFEFB_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FFEFB_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFEFB_PD` reader - FFE and FB simultaneous power-down behaviour"]
pub struct FFEFB_PD_R(crate::FieldReader<bool, FFEFB_PD_A>);
impl FFEFB_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFEFB_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFEFB_PD_A {
        match self.bits {
            false => FFEFB_PD_A::SECUENTIAL,
            true => FFEFB_PD_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == FFEFB_PD_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == FFEFB_PD_A::PARALLEL
    }
}
impl core::ops::Deref for FFEFB_PD_R {
    type Target = crate::FieldReader<bool, FFEFB_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFEFB_PD` writer - FFE and FB simultaneous power-down behaviour"]
pub struct FFEFB_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FFEFB_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFEFB_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If FFE and FB PD event happen at same time, FB will power down first, then FFE."]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(FFEFB_PD_A::SECUENTIAL)
    }
    #[doc = "If FFE and FB PD event happen at same time, FFE and FB will power down in parallel"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(FFEFB_PD_A::PARALLEL)
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
#[doc = "SRAM simultaneous power-down behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PD_A {
    #[doc = "0: If more than one SRAM (M4S1 ~ M4S15) PD event happen at same time, these SRAMs will power down in the following priority. M4S1->M4S2->….->M4S14-> M4S15 (M4S1 has the highest priority and M4S15 has the lowest priority)"]
    SECUENTIAL = 0,
    #[doc = "1: If more than one SRAM (M4S1 ~ M4S15, NOT include M4S0) PD event happen at same time, these SRAMs will power down in parallel."]
    PARALLEL = 1,
}
impl From<SRAM_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PD` reader - SRAM simultaneous power-down behaviour"]
pub struct SRAM_PD_R(crate::FieldReader<bool, SRAM_PD_A>);
impl SRAM_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PD_A {
        match self.bits {
            false => SRAM_PD_A::SECUENTIAL,
            true => SRAM_PD_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == SRAM_PD_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == SRAM_PD_A::PARALLEL
    }
}
impl core::ops::Deref for SRAM_PD_R {
    type Target = crate::FieldReader<bool, SRAM_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_PD` writer - SRAM simultaneous power-down behaviour"]
pub struct SRAM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If more than one SRAM (M4S1 ~ M4S15) PD event happen at same time, these SRAMs will power down in the following priority. M4S1->M4S2->….->M4S14-> M4S15 (M4S1 has the highest priority and M4S15 has the lowest priority)"]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(SRAM_PD_A::SECUENTIAL)
    }
    #[doc = "If more than one SRAM (M4S1 ~ M4S15, NOT include M4S0) PD event happen at same time, these SRAMs will power down in parallel."]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(SRAM_PD_A::PARALLEL)
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
#[doc = "AUDIO simultaneous power-down behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_PD_A {
    #[doc = "0: If more than one AUDIO Blocks (AD0 ~ AD5) PD event happen at same time, these Blocks will power down in the following priority. AD5->AD1->AD2->AD0->AD3->AD4. (AD5 has the highest priority and AD4 has the lowest priority)"]
    SECUENTIAL = 0,
    #[doc = "1: If more than one AUDIO Blocks (AD0 ~ AD5) PD event happen at same time, these AUDIO power domain will power down in parallel"]
    PARALLEL = 1,
}
impl From<AUDIO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDIO_PD` reader - AUDIO simultaneous power-down behaviour"]
pub struct AUDIO_PD_R(crate::FieldReader<bool, AUDIO_PD_A>);
impl AUDIO_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUDIO_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_PD_A {
        match self.bits {
            false => AUDIO_PD_A::SECUENTIAL,
            true => AUDIO_PD_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == AUDIO_PD_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == AUDIO_PD_A::PARALLEL
    }
}
impl core::ops::Deref for AUDIO_PD_R {
    type Target = crate::FieldReader<bool, AUDIO_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUDIO_PD` writer - AUDIO simultaneous power-down behaviour"]
pub struct AUDIO_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "If more than one AUDIO Blocks (AD0 ~ AD5) PD event happen at same time, these Blocks will power down in the following priority. AD5->AD1->AD2->AD0->AD3->AD4. (AD5 has the highest priority and AD4 has the lowest priority)"]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(AUDIO_PD_A::SECUENTIAL)
    }
    #[doc = "If more than one AUDIO Blocks (AD0 ~ AD5) PD event happen at same time, these AUDIO power domain will power down in parallel"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(AUDIO_PD_A::PARALLEL)
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
#[doc = "M4 and M4S0 simultaneous power-down behaviour\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4M4S0_PD_A {
    #[doc = "0: Not defined. guess: First M4 then M4S0"]
    SECUENTIAL = 0,
    #[doc = "1: If M4 and M4S0 PD event happen at same time, then M4 and M4S0 will put into power saving mode at same time"]
    PARALLEL = 1,
}
impl From<M4M4S0_PD_A> for bool {
    #[inline(always)]
    fn from(variant: M4M4S0_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4M4S0_PD` reader - M4 and M4S0 simultaneous power-down behaviour"]
pub struct M4M4S0_PD_R(crate::FieldReader<bool, M4M4S0_PD_A>);
impl M4M4S0_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4M4S0_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4M4S0_PD_A {
        match self.bits {
            false => M4M4S0_PD_A::SECUENTIAL,
            true => M4M4S0_PD_A::PARALLEL,
        }
    }
    #[doc = "Checks if the value of the field is `SECUENTIAL`"]
    #[inline(always)]
    pub fn is_secuential(&self) -> bool {
        **self == M4M4S0_PD_A::SECUENTIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        **self == M4M4S0_PD_A::PARALLEL
    }
}
impl core::ops::Deref for M4M4S0_PD_R {
    type Target = crate::FieldReader<bool, M4M4S0_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4M4S0_PD` writer - M4 and M4S0 simultaneous power-down behaviour"]
pub struct M4M4S0_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4M4S0_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4M4S0_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not defined. guess: First M4 then M4S0"]
    #[inline(always)]
    pub fn secuential(self) -> &'a mut W {
        self.variant(M4M4S0_PD_A::SECUENTIAL)
    }
    #[doc = "If M4 and M4S0 PD event happen at same time, then M4 and M4S0 will put into power saving mode at same time"]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(M4M4S0_PD_A::PARALLEL)
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
impl R {
    #[doc = "Bit 0 - FFE and FB simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn ffefb_wu(&self) -> FFEFB_WU_R {
        FFEFB_WU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn sram_wu(&self) -> SRAM_WU_R {
        SRAM_WU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Audio simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn audio_wu(&self) -> AUDIO_WU_R {
        AUDIO_WU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - M4 and M4S0 simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn m4m4s0_wu(&self) -> M4M4S0_WU_R {
        M4M4S0_WU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FFE and FB simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn ffefb_pd(&self) -> FFEFB_PD_R {
        FFEFB_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SRAM simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn sram_pd(&self) -> SRAM_PD_R {
        SRAM_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUDIO simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn audio_pd(&self) -> AUDIO_PD_R {
        AUDIO_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - M4 and M4S0 simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn m4m4s0_pd(&self) -> M4M4S0_PD_R {
        M4M4S0_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FFE and FB simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn ffefb_wu(&mut self) -> FFEFB_WU_W {
        FFEFB_WU_W { w: self }
    }
    #[doc = "Bit 1 - SRAM simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn sram_wu(&mut self) -> SRAM_WU_W {
        SRAM_WU_W { w: self }
    }
    #[doc = "Bit 2 - Audio simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn audio_wu(&mut self) -> AUDIO_WU_W {
        AUDIO_WU_W { w: self }
    }
    #[doc = "Bit 3 - M4 and M4S0 simultaneous wakeup-event behaviour"]
    #[inline(always)]
    pub fn m4m4s0_wu(&mut self) -> M4M4S0_WU_W {
        M4M4S0_WU_W { w: self }
    }
    #[doc = "Bit 4 - FFE and FB simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn ffefb_pd(&mut self) -> FFEFB_PD_W {
        FFEFB_PD_W { w: self }
    }
    #[doc = "Bit 5 - SRAM simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn sram_pd(&mut self) -> SRAM_PD_W {
        SRAM_PD_W { w: self }
    }
    #[doc = "Bit 6 - AUDIO simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn audio_pd(&mut self) -> AUDIO_PD_W {
        AUDIO_PD_W { w: self }
    }
    #[doc = "Bit 7 - M4 and M4S0 simultaneous power-down behaviour"]
    #[inline(always)]
    pub fn m4m4s0_pd(&mut self) -> M4M4S0_PD_W {
        M4M4S0_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Down Scheme configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_dwn_sch](index.html) module"]
pub struct PWR_DWN_SCH_SPEC;
impl crate::RegisterSpec for PWR_DWN_SCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_dwn_sch::R](R) reader structure"]
impl crate::Readable for PWR_DWN_SCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_dwn_sch::W](W) writer structure"]
impl crate::Writable for PWR_DWN_SCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_DWN_SCH to value 0"]
impl crate::Resettable for PWR_DWN_SCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
