#[doc = "Register `M4_MEM_INT` reader"]
pub struct R(crate::R<M4_MEM_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_MEM_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_MEM_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_MEM_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4_MEM_INT` writer"]
pub struct W(crate::W<M4_MEM_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4_MEM_INT_SPEC>;
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
impl From<crate::W<M4_MEM_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4_MEM_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMO_INTR0` reader - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR0_R(crate::FieldReader<bool, bool>);
impl MEMO_INTR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMO_INTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMO_INTR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMO_INTR0` writer - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMO_INTR0_W<'a> {
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
#[doc = "Field `MEMO_INTR1` reader - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_1) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR1_R(crate::FieldReader<bool, bool>);
impl MEMO_INTR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMO_INTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMO_INTR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMO_INTR1` writer - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_1) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMO_INTR1_W<'a> {
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
#[doc = "Field `MEMO_INTR2` reader - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_2) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR2_R(crate::FieldReader<bool, bool>);
impl MEMO_INTR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMO_INTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMO_INTR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMO_INTR2` writer - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_2) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMO_INTR2_W<'a> {
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
#[doc = "Field `MEMO_INTR3` reader - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_3) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR3_R(crate::FieldReader<bool, bool>);
impl MEMO_INTR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMO_INTR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMO_INTR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMO_INTR3` writer - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_3) while it in deep sleep or shut down mode"]
pub struct MEMO_INTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMO_INTR3_W<'a> {
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
#[doc = "Field `MEM1_INTR0` reader - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR0_R(crate::FieldReader<bool, bool>);
impl MEM1_INTR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM1_INTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM1_INTR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM1_INTR0` writer - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM1_INTR0_W<'a> {
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
#[doc = "Field `MEM1_INTR1` reader - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_1) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR1_R(crate::FieldReader<bool, bool>);
impl MEM1_INTR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM1_INTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM1_INTR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM1_INTR1` writer - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_1) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM1_INTR1_W<'a> {
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
#[doc = "Field `MEM1_INTR2` reader - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_2) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR2_R(crate::FieldReader<bool, bool>);
impl MEM1_INTR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM1_INTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM1_INTR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM1_INTR2` writer - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_2) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM1_INTR2_W<'a> {
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
#[doc = "Field `MEM1_INTR3` reader - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_3) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR3_R(crate::FieldReader<bool, bool>);
impl MEM1_INTR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM1_INTR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM1_INTR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM1_INTR3` writer - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_3) while it in deep sleep or shut down mode"]
pub struct MEM1_INTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM1_INTR3_W<'a> {
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
#[doc = "Field `MEM2_INTR0` reader - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR0_R(crate::FieldReader<bool, bool>);
impl MEM2_INTR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM2_INTR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM2_INTR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM2_INTR0` writer - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM2_INTR0_W<'a> {
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
#[doc = "Field `MEM2_INTR1` reader - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_1) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR1_R(crate::FieldReader<bool, bool>);
impl MEM2_INTR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM2_INTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM2_INTR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM2_INTR1` writer - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_1) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM2_INTR1_W<'a> {
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
#[doc = "Field `MEM2_INTR2` reader - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_2) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR2_R(crate::FieldReader<bool, bool>);
impl MEM2_INTR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM2_INTR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM2_INTR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM2_INTR2` writer - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_2) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM2_INTR2_W<'a> {
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
#[doc = "Field `MEM2_INTR3` reader - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_3) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR3_R(crate::FieldReader<bool, bool>);
impl MEM2_INTR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM2_INTR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM2_INTR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM2_INTR3` writer - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_3) while it in deep sleep or shut down mode"]
pub struct MEM2_INTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM2_INTR3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr0(&self) -> MEMO_INTR0_R {
        MEMO_INTR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_1) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr1(&self) -> MEMO_INTR1_R {
        MEMO_INTR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_2) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr2(&self) -> MEMO_INTR2_R {
        MEMO_INTR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_3) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr3(&self) -> MEMO_INTR3_R {
        MEMO_INTR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr0(&self) -> MEM1_INTR0_R {
        MEM1_INTR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_1) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr1(&self) -> MEM1_INTR1_R {
        MEM1_INTR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_2) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr2(&self) -> MEM1_INTR2_R {
        MEM1_INTR2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_3) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr3(&self) -> MEM1_INTR3_R {
        MEM1_INTR3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr0(&self) -> MEM2_INTR0_R {
        MEM2_INTR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_1) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr1(&self) -> MEM2_INTR1_R {
        MEM2_INTR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_2) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr2(&self) -> MEM2_INTR2_R {
        MEM2_INTR2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_3) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr3(&self) -> MEM2_INTR3_R {
        MEM2_INTR3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr0(&mut self) -> MEMO_INTR0_W {
        MEMO_INTR0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_1) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr1(&mut self) -> MEMO_INTR1_W {
        MEMO_INTR1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_2) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr2(&mut self) -> MEMO_INTR2_W {
        MEMO_INTR2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt caused by a SRAM access (M4 SRAM segment 0 32KB_3) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn memo_intr3(&mut self) -> MEMO_INTR3_W {
        MEMO_INTR3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr0(&mut self) -> MEM1_INTR0_W {
        MEM1_INTR0_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_1) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr1(&mut self) -> MEM1_INTR1_W {
        MEM1_INTR1_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_2) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr2(&mut self) -> MEM1_INTR2_W {
        MEM1_INTR2_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt caused by a SRAM access (M4 SRAM segment 1 32KB_3) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem1_intr3(&mut self) -> MEM1_INTR3_W {
        MEM1_INTR3_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_0) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr0(&mut self) -> MEM2_INTR0_W {
        MEM2_INTR0_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_1) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr1(&mut self) -> MEM2_INTR1_W {
        MEM2_INTR1_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_2) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr2(&mut self) -> MEM2_INTR2_W {
        MEM2_INTR2_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt caused by a SRAM access (M4 SRAM segment 2 32KB_3) while it in deep sleep or shut down mode"]
    #[inline(always)]
    pub fn mem2_intr3(&mut self) -> MEM2_INTR3_W {
        MEM2_INTR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM access while in low power mode interrupt flag register (Set bit to clear)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_mem_int](index.html) module"]
pub struct M4_MEM_INT_SPEC;
impl crate::RegisterSpec for M4_MEM_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_mem_int::R](R) reader structure"]
impl crate::Readable for M4_MEM_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4_mem_int::W](W) writer structure"]
impl crate::Writable for M4_MEM_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4_MEM_INT to value 0"]
impl crate::Resettable for M4_MEM_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
