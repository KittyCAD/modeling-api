#!/usr/bin/awk -f

# This awk script adds #[cfg_attr(feature = "fuzzing", derive(arbitrary::Arbitrary))]
# after any line containing #[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
# while preserving the original indentation

{
    # Print the current line
    print $0
    
    # Check if the line contains the ts-rs derive attribute
    if ($0 ~ /#\[cfg_attr\(feature = "ts-rs", derive\(ts_rs::TS\)\)\]/) {
        # Extract the indentation (everything before the first non-space character)
        match($0, /^[ \t]*/)
        indentation = substr($0, 1, RLENGTH)
        
        # Print the fuzzing attribute with the same indentation
        print indentation "#[cfg_attr(feature = \"fuzzing\", derive(arbitrary::Arbitrary))]"
    }
}