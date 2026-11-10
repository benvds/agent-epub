-- Pandoc combines supplied Markdown files. Convert source-relative links to
-- supplied source headings into fragments before its EPUB writer splits them.
function Link(link)
  local path, fragment = link.target:match('^([^#]+)#(.+)$')
  if path and fragment then
    for _, input in ipairs(PANDOC_STATE.input_files) do
      local basename = input:match('([^/]+)$')
      if path == input or path == basename then
        link.target = '#' .. fragment
        return link
      end
    end
  end
end
