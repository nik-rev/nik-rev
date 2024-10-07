import {
  Children,
  cloneElement,
  type HTMLAttributes,
  isValidElement,
  type ReactElement,
} from "react";
import { GoDash } from "react-icons/go";
import { RiDoubleQuotesL, RiDoubleQuotesR } from "react-icons/ri";

import { coloredText } from "@/lib/admonition-accent";
import { cn } from "@/lib/utils";

export function BlockQuote({
  children,
  credit,
  ...props
}: HTMLAttributes<HTMLQuoteElement> & { credit?: React.ReactNode }) {
  const childrenArray = Children.toArray(children);

  const lastChildIndex = childrenArray.findLastIndex((el) =>
    isValidElement(el),
  );

  if (lastChildIndex === -1) {
    throw new Error("Expected an element to be contained within Blockquote");
  }

  const lastChild = childrenArray.at(lastChildIndex) as ReactElement;
  const lastChildWithIcon = cloneElement(lastChild, {
    children: (
      <>
        {/* eslint @typescript-eslint/no-unsafe-member-access: off -- props is expected to contain children */}
        {lastChild.props.children}{" "}
        <span className="relative">
          <RiDoubleQuotesR
            className={`absolute -top-2 left-1 inline align-baseline text-2xl text-green ${coloredText}`}
          />
        </span>
      </>
    ),
  });

  const childrenWithIcon = childrenArray.with(
    lastChildIndex,
    lastChildWithIcon,
  );

  const quoteContent = (
    <div className="relative ml-4">
      {childrenWithIcon}
      <RiDoubleQuotesL
        className={`absolute -left-6 top-[-0.2rem] text-2xl text-green ${coloredText}`}
      />
    </div>
  );

  const className = `block overflow-x-auto bg-lavender/5 italic max-sm:text-sm bleed`;

  const blockQuote = (
    <blockquote className={className} {...props}>
      {quoteContent}
    </blockquote>
  );

  const blockQuoteWithCredit = (
    <figure className={cn(className, "space-y-2 md:space-y-6")}>
      <blockquote {...props}>{quoteContent}</blockquote>
      <figcaption className="flex items-center not-italic text-subtext0">
        <GoDash className="mr-2 inline" strokeWidth={1} />
        {credit}
      </figcaption>
    </figure>
  );

  return credit ? blockQuoteWithCredit : blockQuote;
}