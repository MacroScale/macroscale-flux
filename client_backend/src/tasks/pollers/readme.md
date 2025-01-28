# Pollers

## Overview

This is a collection of tasks used to poll data from various sources.

## Poll thread sleep time

Polls all run on separate threads, polling works by constantly looping the same code block.
The general time for polling is **100ms**, however, the event poller (poll_events) needs 
to run faster than this to ensure that event work is carried out before another poll is run. 
This is to avoid duplicate events being dispatched.

e.g. 
- if poll_events ticked at **100ms**
    - poll_foreground_window dispatches an event because foreground_window has changed
    - event work is being carried out of setting the app_data foreground window to new window
    - poll_foreground_window dispatches the same event because foreground_window has not updated yet
    - event work complete








