use std::io::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;

fn name_signal(num: u32) -> std::io::Result<String>
{
   let kill_cmd = Command::new("kill").args(["-l", &num.to_string()]).output()?;
   if !kill_cmd.status.success()
   {
      return Err(Error::new(ErrorKind::ConnectionReset, String::from_utf8(kill_cmd.stderr).unwrap_or(String::from("Unknown error"))));
   }
   let result = match String::from_utf8(kill_cmd.stdout)
   {
      Ok(r) => r,
      Err(_) => { return Err(Error::new(ErrorKind::InvalidData, "Conversion has failed")) },
   };
   Ok(String::from("SIG") + &result)
}

fn get_signals(s: &str) -> Vec<u32>
{
   let mut result = Vec::<u32>::new();
   let mut it_v: Vec<char> = s.chars().collect();
   it_v.reverse();
   let mut it = it_v.into_iter();
   let mut offset: u32 = 0;
   while let Some(n_str) = it.next()
   {
      if n_str == '1' || n_str == '3' || n_str == '5' || n_str == '7' || n_str == '9' || n_str == 'b' || n_str == 'd' || n_str == 'f'
      {
         result.push(offset+1);
      }
      if n_str == '2' || n_str == '3' || n_str == '6' || n_str == '7' || n_str == 'a' || n_str == 'b' || n_str == 'e' || n_str == 'f'
      {
         result.push(offset+2);
      }
      if n_str == '4' || n_str == '5' || n_str == '6' || n_str == '7' || n_str == 'c' || n_str == 'd' || n_str == 'e' || n_str == 'f'
      {
         result.push(offset+4);
      }
      if n_str == '8' || n_str == '9' || n_str == 'a' || n_str == 'b' || n_str == 'c' || n_str == 'd' || n_str == 'e' || n_str == 'f'
      {
         result.push(offset+8);
      }
      offset += 4;
   } // */
   result
}

fn main() -> std::io::Result<()> {
   let mut arg = std::env::args();
   let program_name = arg.next().unwrap(); // if there is no first argument, something is wrong
   let str_pid = match arg.next()
   {
      Some(n) => n,
      None => {
         println!("Usage: {} <PID>", program_name);
         return Ok(())
      },
   };
   if str_pid.starts_with("-h") || str_pid.starts_with("--help")
   {
      println!("Usage: {} <PID>", program_name);
      return Ok(())
   }
   if str_pid.starts_with("-d") || str_pid.starts_with("--decode")
   {
      // with -d, the signal mask is supplied as a separate argument
      // with --decode, the signal mask may be supplied either as a separate argument or as a part of the parameter
      let signal_mask = if str_pid.starts_with("-d")
      {
         arg.next().expect("Please, supply a signal mask after the \"-d\" argument!")
      }
      else
      {
         arg.next().unwrap_or_else(||
         {
            str_pid.strip_prefix("--decode=").expect("Please, supply a signal mask after the \"--decode\" argument!").to_string()
         })
      };
      let signal_vector = get_signals(&signal_mask);
      for signal in signal_vector
      {
         let mut printable_name = name_signal(signal)?;
         printable_name = printable_name.trim().to_string();
         if printable_name.len() > 3
         {
            println!("{}", printable_name);
         }
      }
      return Ok(())
   }
   let selected = i64::from_str_radix(&str_pid, 10).unwrap();
   let mut status_file = BufReader::new(File::open(format!("/proc/{}/status", selected))?).lines();
   let mut caught = Vec::<u32>::with_capacity(64);
   let mut ignored = Vec::<u32>::with_capacity(64);
   let mut blocked = Vec::<u32>::with_capacity(64);
   let mut thread_pending = Vec::<u32>::with_capacity(64);
   let mut process_pending = Vec::<u32>::with_capacity(64);
   while let Some(l) = status_file.next()
   {
      let line = l?;
      if line.starts_with("SigCgt:")
      {
         caught = get_signals(line.strip_prefix("SigCgt:").expect("An error has occurred while parsing the list of caught signals for this process"));
      }
      if line.starts_with("SigIgn:")
      {
         ignored = get_signals(line.strip_prefix("SigIgn:").expect("An error has occurred while parsing the list of ignored signals for this process"));
      }
      if line.starts_with("SigBlk:")
      {
         blocked = get_signals(line.strip_prefix("SigBlk:").expect("An error has occurred while parsing the list of blocked signals for this process"));
      }
      if line.starts_with("SigPnd:")
      {
         thread_pending = get_signals(line.strip_prefix("SigPnd:").expect("An error has occurred while parsing the list of pending signals for this thread"));
      }
      if line.starts_with("ShdPnd:")
      {
         process_pending = get_signals(line.strip_prefix("ShdPnd:").expect("An error has occurred while parsing the list of pending signals for this process"));
      }
   }
   println!("Signals caught by {}:", str_pid);
   for signal in caught
   {
      let name = name_signal(signal)?;
      if name.len() > 3
      {
         println!("{}", name.trim());
      }
   }
   println!("Signals ignored by {}:", str_pid);
   for signal in ignored
   {
      let name = name_signal(signal)?;
      if name.len() > 3
      {
         println!("{}", name.trim());
      }
   }
   println!("Signals blocked by {}:", str_pid);
   for signal in blocked
   {
      let name = name_signal(signal)?;
      if name.len() > 3
      {
         println!("{}", name.trim());
      }
   }
   println!("Signals pending for {}:", str_pid);
   for signal in thread_pending
   {
      let name = name_signal(signal)?;
      if name.len() > 3
      {
         println!("{}", name.trim());
      }
   }
   println!("Signals pending for the whole process of {}:", str_pid);
   for signal in process_pending
   {
      let name = name_signal(signal)?;
      if name.len() > 3
      {
         println!("{}", name.trim());
      }
   }
   Ok(())
}
