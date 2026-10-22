#import <AppKit/AppKit.h>
#include "Standalone.h"

namespace sadpet::mac {
namespace {
void unregisterDrops(NSView* view) {
    [view unregisterDraggedTypes];
    for (NSView* child in view.subviews) unregisterDrops(child);
}

void visit(juce::Component& component) {
    if (auto* embedded = dynamic_cast<juce::NSViewComponent*>(&component))
        if (auto* view = static_cast<NSView*>(embedded->getView())) unregisterDrops(view);
    for (auto* child : component.getChildren()) visit(*child);
}
}

void routeFileDropsToApp(juce::Component& root) { visit(root); }

int webViewDropTypes(juce::Component& root) {
    int count = 0;
    std::function<void(NSView*)> countView = [&](NSView* view) {
        count += static_cast<int>(view.registeredDraggedTypes.count);
        for (NSView* child in view.subviews) countView(child);
    };
    std::function<void(juce::Component&)> search = [&](juce::Component& component) {
        if (auto* embedded = dynamic_cast<juce::NSViewComponent*>(&component))
            if (auto* view = static_cast<NSView*>(embedded->getView())) countView(view);
        for (auto* child : component.getChildren()) search(*child);
    };
    search(root);
    return count;
}

void installEditShortcuts() {
    static id monitor = nil;
    if (monitor != nil) return;
    monitor = [NSEvent addLocalMonitorForEventsMatchingMask: NSEventMaskKeyDown handler: ^NSEvent*(NSEvent* event) {
        const auto flags = event.modifierFlags & NSEventModifierFlagDeviceIndependentFlagsMask;
        if ((flags & NSEventModifierFlagCommand) == 0
            || (flags & (NSEventModifierFlagControl | NSEventModifierFlagOption)) != 0)
            return event;
        NSString* key = event.charactersIgnoringModifiers.lowercaseString;
        const auto shift = (flags & NSEventModifierFlagShift) != 0;
        SEL action = nil;
        if ([key isEqualToString: @"x"]) action = @selector(cut:);
        else if ([key isEqualToString: @"c"]) action = @selector(copy:);
        else if ([key isEqualToString: @"v"]) action = @selector(paste:);
        else if ([key isEqualToString: @"a"]) action = @selector(selectAll:);
        else if ([key isEqualToString: @"z"]) action = shift ? @selector(redo:) : @selector(undo:);
        if (action != nil && [NSApp sendAction: action to: nil from: nil]) return nil;
        return event;
    }];
   #if ! __has_feature(objc_arc)
    [monitor retain];
   #endif
}
}
