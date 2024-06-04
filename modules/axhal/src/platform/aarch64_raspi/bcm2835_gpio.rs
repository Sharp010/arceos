use crate::mem::phys_to_virt;
use bcm2835_gpio::RpiGpio;
use memory_addr::PhysAddr;
use spinlock::SpinNoIrq;
use axconfig::GPIO_PADDR; 


// 物理地址 
const GPIO_PHY: PhysAddr = PhysAddr::from(GPIO_PADDR); 

// 创建一个GPIO的SpinNoIrq
static GPIO: SpinNoIrq<RpiGpio> = 
SpinNoIrq::new(RpiGpio::new( phys_to_virt(GPIO_PHY).as_mut_ptr() ));


/// led simply initialize
pub fn init_led() {
    info!("Initialize Led...");
    GPIO.lock().gpio_init();
}