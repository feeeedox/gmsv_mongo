import { defineContentConfig, defineCollection } from '@nuxt/content'
import { z } from 'zod'
export default defineContentConfig({
  collections: {
    content: defineCollection({
      type: 'page',
      source: {
        include: '**',
        exclude: ['**/.!(navigation.yml)']
      },
      schema: z.object({
        category: z.string().optional(),
        navigation: z.object({
          title: z.string().optional(),
          order: z.number().optional(),
          hide: z.boolean().optional()
        }).optional()
      })
    })
  }
})
